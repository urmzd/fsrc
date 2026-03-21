use crossterm::style::Stylize;
use std::io::{self, Write};

/// Print a cyan bold header with a 40-char dim rule beneath it.
#[allow(dead_code)]
pub fn header(text: &str) {
    let mut err = io::stderr().lock();
    let rule = "─".repeat(40);
    let _ = writeln!(err, "  {}", text.cyan().bold());
    let _ = writeln!(err, "  {}", rule.dim());
}

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
