use regex::Regex;
use std::path::Path;

use crate::lang::ext_to_lang;
use crate::ui;

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

/// Parse a `lines` attribute value and extract the matching lines from content.
///
/// Supported formats (all 1-indexed):
///   - `"5"` — single line 5
///   - `"5-10"` — lines 5 through 10 (inclusive)
///   - `"5-"` — line 5 through end of file
///   - `"-10"` — line 1 through 10
fn extract_lines(content: &str, spec: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let total = lines.len();

    let (start, end) = if let Some((left, right)) = spec.split_once('-') {
        let s = if left.is_empty() {
            1
        } else {
            left.parse::<usize>().unwrap_or(1)
        };
        let e = if right.is_empty() {
            total
        } else {
            right.parse::<usize>().unwrap_or(total)
        };
        (s, e)
    } else {
        // Single line number.
        let n = spec.parse::<usize>().unwrap_or(1);
        (n, n)
    };

    // Clamp to valid range.
    let start = start.max(1).min(total + 1);
    let end = end.max(start).min(total);

    if start > total {
        return String::new();
    }

    lines[(start - 1)..end].join("\n")
}

/// Result of processing a single file.
pub struct ProcessResult {
    pub original: String,
    pub processed: String,
}

/// Process a file: find all `embed-src src="..."` directives and replace the
/// content between them and their closing `/embed-src` markers.
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
/// `embed-src src="path"` is an opening marker, and any line containing
/// `/embed-src` is a closing marker. This allows embedding in any file type
/// (markdown, Rust, Python, YAML, etc.).
///
/// By default, content is inserted raw. Use the `fence` attribute to wrap in
/// markdown code fences: `fence` or `fence="auto"` auto-detects the language
/// from the source extension; `fence="python"` uses an explicit language tag.
pub fn process_content(content: &str, base_dir: &Path) -> String {
    let open_re = Regex::new(r#"embed-src\s+src="([^"]+)""#).unwrap();
    let lines_re = Regex::new(r#"lines="([^"]+)""#).unwrap();
    let fence_re = Regex::new(r#"\bfence(?:="([^"]*)")?"#).unwrap();
    // Match /embed-src preceded by a non-word character (space, comment chars, etc.)
    // but not as part of a URL like "urmzd/embed-src".
    let close_re = Regex::new(r#"(?:^|[^a-zA-Z0-9_])/embed-src\b"#).unwrap();

    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut i = 0;
    let has_trailing_newline = content.ends_with('\n');
    let mut in_fence = false;
    let mut fence_len: usize = 0;

    while i < lines.len() {
        let line = lines[i];

        // Track backtick-fenced code blocks so directives inside them are skipped.
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            let backtick_count = trimmed.bytes().take_while(|&b| b == b'`').count();
            if !in_fence {
                in_fence = true;
                fence_len = backtick_count;
                result.push(line.to_string());
                i += 1;
                continue;
            } else if backtick_count >= fence_len {
                in_fence = false;
                fence_len = 0;
                result.push(line.to_string());
                i += 1;
                continue;
            }
        }

        if in_fence {
            result.push(line.to_string());
            i += 1;
            continue;
        }

        if let Some(cap) = open_re.captures(line) {
            let src_path = cap[1].to_string();
            let lines_attr = lines_re.captures(line).map(|c| c[1].to_string());
            let fence_cap = fence_re.captures(line);
            let has_fence = fence_cap.is_some();
            let fence_attr = fence_cap.and_then(|c| c.get(1).map(|m| m.as_str().to_string()));

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
                ui::warn(&format!(
                    "no closing /embed-src found for directive at line {}",
                    i + 1
                ));
                i += 1;
                continue;
            }

            // Read source file.
            let file_path = base_dir.join(&src_path);
            let file_content = match std::fs::read_to_string(&file_path) {
                Ok(c) => c,
                Err(e) => {
                    ui::warn(&format!("could not read {}: {}", file_path.display(), e));
                    // Emit original lines unchanged.
                    for line in &lines[(i + 1)..=close_line_idx] {
                        result.push(line.to_string());
                    }
                    i = close_line_idx + 1;
                    continue;
                }
            };

            // Apply line-range filter if specified.
            let file_content = match &lines_attr {
                Some(spec) => extract_lines(&file_content, spec),
                None => file_content,
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
        let input = "<!-- embed-src src=\"foo.rs\" -->\nstale content\n";
        let result = process_content(input, Path::new("."));
        // Should leave content unchanged when no closing tag.
        assert_eq!(result, input);
    }

    #[test]
    fn extract_lines_single() {
        let content = "line1\nline2\nline3\n";
        assert_eq!(extract_lines(content, "2"), "line2");
    }

    #[test]
    fn extract_lines_range() {
        let content = "a\nb\nc\nd\ne\n";
        assert_eq!(extract_lines(content, "2-4"), "b\nc\nd");
    }

    #[test]
    fn extract_lines_open_end() {
        let content = "a\nb\nc\nd\n";
        assert_eq!(extract_lines(content, "3-"), "c\nd");
    }

    #[test]
    fn extract_lines_open_start() {
        let content = "a\nb\nc\nd\n";
        assert_eq!(extract_lines(content, "-2"), "a\nb");
    }

    #[test]
    fn extract_lines_out_of_bounds() {
        let content = "a\nb\nc\n";
        // End beyond file length: clamp to last line.
        assert_eq!(extract_lines(content, "2-100"), "b\nc");
        // Start beyond file length: empty.
        assert_eq!(extract_lines(content, "100"), "");
    }
}
