use crossterm::style::Stylize;
use std::io::{self, Write};

/// Print a green bold checkmark with a message.
pub fn phase_ok(text: &str) {
    let mut err = io::stderr().lock();
    let _ = writeln!(err, "  {} {}", "✓".green().bold(), text);
}

/// Print a yellow bold warning.
pub fn warn(text: &str) {
    let mut err = io::stderr().lock();
    let _ = writeln!(err, "  {} {}", "⚠".yellow().bold(), text.yellow());
}

/// Print a cyan info marker with dim text.
pub fn info(text: &str) {
    let mut err = io::stderr().lock();
    let _ = writeln!(err, "  {} {}", "ℹ".cyan(), text.dim());
}

/// Print a red bold error.
pub fn error(text: &str) {
    let mut err = io::stderr().lock();
    let _ = writeln!(err, "  {} {}", "✗".red().bold(), text.red());
}
