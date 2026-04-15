mod self_update;

use std::path::Path;
use std::process;

use clap::{Parser, Subcommand};

use fsrc::embed::process_file;
use fsrc::ui;

#[derive(Parser)]
#[command(name = "fsrc", about = "Embed source files into any text file")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Args)]
struct RunArgs {
    /// Files to process
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

#[derive(Subcommand)]
enum Command {
    /// Embed source files into text files
    Run(RunArgs),
    /// Update fsrc to the latest release
    Update,
    /// Print version
    Version,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Run(args) => {
            if args.files.is_empty() {
                ui::error("no files specified");
                process::exit(2);
            }

            if args.output.is_some() && args.files.len() > 1 {
                ui::error("--output can only be used with a single input file");
                process::exit(2);
            }

            run_embed(&args);
        }
        Command::Update => {
            eprintln!("current version: {}", env!("CARGO_PKG_VERSION"));
            match self_update::self_update("urmzd/fsrc", env!("CARGO_PKG_VERSION"), "fsrc") {
                Ok(self_update::UpdateResult::AlreadyUpToDate) => {
                    eprintln!("already up to date")
                }
                Ok(self_update::UpdateResult::Updated { from, to }) => {
                    eprintln!("updated: {from} → {to}")
                }
                Err(e) => {
                    ui::error(&format!("update failed: {e}"));
                    process::exit(1);
                }
            }
        }
        Command::Version => {
            println!("fsrc {}", env!("CARGO_PKG_VERSION"));
        }
    }
}

fn run_embed(args: &RunArgs) {
    let mut needs_update = false;
    let mut had_error = false;

    for file in &args.files {
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
            if !args.verify && !args.dry_run {
                ui::phase_ok(&format!("{} is up to date", file));
            }
            continue;
        }

        needs_update = true;

        if args.verify {
            ui::warn(&format!("{} is out of date", file));
            continue;
        }

        if args.dry_run {
            ui::info(&format!("{} would be updated", file));
            continue;
        }

        let dest = args.output.as_deref().unwrap_or(file);
        if let Err(e) = std::fs::write(dest, &result.processed) {
            ui::error(&format!("writing {}: {}", dest, e));
            had_error = true;
        } else if dest == file {
            ui::phase_ok(&format!("{} updated", file));
        } else {
            ui::phase_ok(&format!("{} → {}", file, dest));
        }
    }

    if had_error {
        process::exit(2);
    }

    if args.verify && needs_update {
        process::exit(1);
    }
}
