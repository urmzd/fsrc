use std::path::Path;
use std::process;

use clap::Parser;

use embed_src::embed::process_file;
use embed_src::ui;

#[derive(Parser)]
#[command(name = "embed-src", about = "Embed source files into any text file")]
struct Cli {
    /// Files to process
    #[arg(required = true)]
    files: Vec<String>,

    /// Check if files are up-to-date (exit 1 if changes needed)
    #[arg(long)]
    verify: bool,

    /// Print what would change without modifying files
    #[arg(long)]
    dry_run: bool,

    /// Write output to a file instead of modifying in place
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if cli.output.is_some() && cli.files.len() > 1 {
        ui::error("--output can only be used with a single input file");
        process::exit(2);
    }

    let mut needs_update = false;
    let mut had_error = false;

    for file in &cli.files {
        let path = Path::new(file);

        let result = match process_file(path) {
            Ok(r) => r,
            Err(e) => {
                ui::error(&e);
                had_error = true;
                continue;
            }
        };

        if result.original == result.processed {
            if !cli.verify && !cli.dry_run {
                ui::phase_ok(&format!("{} is up to date", file));
            }
            continue;
        }

        needs_update = true;

        if cli.verify {
            ui::warn(&format!("{} is out of date", file));
            continue;
        }

        if cli.dry_run {
            ui::info(&format!("{} would be updated", file));
            continue;
        }

        let dest = cli.output.as_deref().unwrap_or(file);
        if let Err(e) = std::fs::write(dest, &result.processed) {
            ui::error(&format!("writing {}: {}", dest, e));
            had_error = true;
        } else {
            if dest == file {
                ui::phase_ok(&format!("{} updated", file));
            } else {
                ui::phase_ok(&format!("{} → {}", file, dest));
            }
        }
    }

    if had_error {
        process::exit(2);
    }

    if cli.verify && needs_update {
        process::exit(1);
    }
}
