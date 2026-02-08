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

/// Process a markdown file: find all `<!-- embedmd src="..." -->` directives
/// and replace the content between them and their closing `<!-- /embedmd -->` tags.
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

/// Process markdown content, resolving source paths relative to `base_dir`.
pub fn process_content(content: &str, base_dir: &Path) -> String {
    let open_re = Regex::new(r#"<!-- embedmd src="([^"]+)" -->"#).unwrap();
    let close_tag = "<!-- /embedmd -->";

    // Collect all directives first so we can process bottom-to-top.
    let mut directives: Vec<(usize, usize, String)> = Vec::new();

    for cap in open_re.captures_iter(content) {
        let full_match = cap.get(0).unwrap();
        let src_path = cap[1].to_string();
        let open_start = full_match.start();
        let open_end = full_match.end();

        // Find the closing tag after this opening tag.
        let rest = &content[open_end..];
        let close_pos = match rest.find(close_tag) {
            Some(pos) => open_end + pos,
            None => {
                eprintln!(
                    "Warning: no closing <!-- /embedmd --> found for directive at offset {}",
                    open_start
                );
                continue;
            }
        };

        directives.push((open_end, close_pos, src_path));
    }

    // Process bottom-to-top so indices stay valid.
    directives.reverse();

    let mut result = content.to_string();

    for (open_end, close_pos, src_path) in directives {
        let file_path = base_dir.join(&src_path);

        let file_content = match std::fs::read_to_string(&file_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Warning: could not read {}: {}", file_path.display(), e);
                continue;
            }
        };

        let ext = Path::new(&src_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        let lang = ext_to_lang(ext);

        let fence = make_fence(&file_content);
        let replacement = format!(
            "\n{}{}\n{}\n{}\n",
            fence,
            lang,
            file_content.trim_end(),
            fence
        );

        result.replace_range(open_end..close_pos, &replacement);
    }

    result
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
        let input = "<!-- embedmd src=\"foo.rs\" -->\nstale content\n";
        let result = process_content(input, Path::new("."));
        // Should leave content unchanged when no closing tag.
        assert_eq!(result, input);
    }
}
