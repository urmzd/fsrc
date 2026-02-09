use std::path::Path;
use std::process;

use clap::Parser;

use embed_it::embed::process_file;

#[derive(Parser)]
#[command(name = "embed-it", about = "Embed source files into any text file")]
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
}

fn main() {
    let cli = Cli::parse();
    let mut needs_update = false;
    let mut had_error = false;

    for file in &cli.files {
        let path = Path::new(file);

        let result = match process_file(path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error: {}", e);
                had_error = true;
                continue;
            }
        };

        if result.original == result.processed {
            continue;
        }

        needs_update = true;

        if cli.verify {
            eprintln!("{} is out of date", file);
            continue;
        }

        if cli.dry_run {
            eprintln!("{} would be updated", file);
            continue;
        }

        if let Err(e) = std::fs::write(path, &result.processed) {
            eprintln!("Error writing {}: {}", file, e);
            had_error = true;
        }
    }

    if had_error {
        process::exit(2);
    }

    if cli.verify && needs_update {
        process::exit(1);
    }
}
