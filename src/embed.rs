use regex::Regex;
use std::path::Path;

use crate::lang::ext_to_lang;

/// Return a backtick fence long enough to avoid collisions with backtick runs in `content`.
fn make_fence(content: &str) -> String {
    let max_run = content
        .as_bytes()
        .split(|&b| b != b'`')
        .map(|run| run.len())
        .max()
        .unwrap_or(0);
    let fence_len = if max_run >= 3 { max_run + 1 } else { 3 };
    "`".repeat(fence_len)
}

/// Result of processing a single file.
pub struct ProcessResult {
    pub original: String,
    pub processed: String,
}

/// Process a file: find all `embed-it src="..."` directives and replace the
/// content between them and their closing `/embed-it` markers.
pub fn process_file(path: &Path) -> Result<ProcessResult, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    let base_dir = path.parent().unwrap_or(Path::new("."));
    let processed = process_content(&content, base_dir);

    Ok(ProcessResult {
        original: content,
        processed,
    })
}

/// Process content, resolving source paths relative to `base_dir`.
///
/// Markers are comment-agnostic: any line containing
/// `embed-it src="path"` is an opening marker, and any line containing
/// `/embed-it` is a closing marker. This allows embedding in any file type
/// (markdown, Rust, Python, YAML, etc.).
///
/// By default, content is inserted raw. Use the `fence` attribute to wrap in
/// markdown code fences: `fence` or `fence="auto"` auto-detects the language
/// from the source extension; `fence="python"` uses an explicit language tag.
pub fn process_content(content: &str, base_dir: &Path) -> String {
    let open_re = Regex::new(r#"embed-it\s+src="([^"]+)"(?:\s+fence(?:="([^"]*)")?)?"#).unwrap();
    // Match /embed-it preceded by a non-word character (space, comment chars, etc.)
    // but not as part of a URL like "urmzd/embed-it".
    let close_re = Regex::new(r#"(?:^|[^a-zA-Z0-9_])/embed-it\b"#).unwrap();

    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut i = 0;
    let has_trailing_newline = content.ends_with('\n');

    while i < lines.len() {
        let line = lines[i];

        if let Some(cap) = open_re.captures(line) {
            let src_path = cap[1].to_string();
            let fence_attr = cap.get(2).map(|m| m.as_str().to_string());
            // Distinguish fence (no value) vs fence="value" vs no fence at all.
            // Group 0 full match tells us if "fence" appeared.
            let has_fence = cap.get(0).unwrap().as_str().contains("fence");

            // Emit the opening marker line.
            result.push(line.to_string());

            // Skip lines until we find the closing marker or run out of lines.
            let mut found_close = false;
            let mut close_line_idx = i + 1;
            while close_line_idx < lines.len() {
                if close_re.is_match(lines[close_line_idx]) {
                    found_close = true;
                    break;
                }
                close_line_idx += 1;
            }

            if !found_close {
                // No closing marker: emit remaining lines unchanged.
                eprintln!(
                    "Warning: no closing /embed-it found for directive at line {}",
                    i + 1
                );
                i += 1;
                continue;
            }

            // Read source file.
            let file_path = base_dir.join(&src_path);
            let file_content = match std::fs::read_to_string(&file_path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Warning: could not read {}: {}", file_path.display(), e);
                    // Emit original lines unchanged.
                    for line in &lines[(i + 1)..=close_line_idx] {
                        result.push(line.to_string());
                    }
                    i = close_line_idx + 1;
                    continue;
                }
            };

            // Insert content: raw or fenced.
            if has_fence {
                let lang = match &fence_attr {
                    Some(lang) if !lang.is_empty() && lang != "auto" => lang.to_string(),
                    _ => {
                        // auto-detect from extension
                        let ext = Path::new(&src_path)
                            .extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("");
                        ext_to_lang(ext).to_string()
                    }
                };
                let fence = make_fence(&file_content);
                result.push(format!("{}{}", fence, lang));
                result.push(file_content.trim_end().to_string());
                result.push(fence);
            } else {
                // Raw insertion.
                let trimmed = file_content.trim_end();
                if !trimmed.is_empty() {
                    result.push(trimmed.to_string());
                }
            }

            // Emit the closing marker line.
            result.push(lines[close_line_idx].to_string());
            i = close_line_idx + 1;
        } else {
            result.push(line.to_string());
            i += 1;
        }
    }

    let mut output = result.join("\n");
    if has_trailing_newline {
        output.push('\n');
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn no_directives() {
        let input = "# Hello\n\nSome text.\n";
        let result = process_content(input, Path::new("."));
        assert_eq!(result, input);
    }

    #[test]
    fn missing_close_tag() {
        let input = "<!-- embed-it src=\"foo.rs\" -->\nstale content\n";
        let result = process_content(input, Path::new("."));
        // Should leave content unchanged when no closing tag.
        assert_eq!(result, input);
    }
}
