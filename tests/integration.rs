use std::path::Path;

use embed_it::embed::{process_content, process_file};

fn fixtures_dir() -> &'static Path {
    Path::new("tests/fixtures")
}

#[test]
fn single_directive_fenced() {
    let input = "\
# Sample

<!-- embed-it src=\"example.rs\" fence=\"auto\" -->
<!-- /embed-it -->

Done.
";
    let expected = "\
# Sample

<!-- embed-it src=\"example.rs\" fence=\"auto\" -->
```rust
fn main() {
    println!(\"Hello, world!\");
}
```
<!-- /embed-it -->

Done.
";
    let result = process_content(input, fixtures_dir());
    assert_eq!(result, expected);
}

#[test]
fn multiple_directives() {
    let input = "\
<!-- embed-it src=\"example.rs\" fence=\"auto\" -->
<!-- /embed-it -->

Middle text.

<!-- embed-it src=\"example.rs\" fence=\"auto\" -->
<!-- /embed-it -->
";
    let result = process_content(input, fixtures_dir());
    assert!(result.contains("```rust"));
    assert_eq!(result.matches("```rust").count(), 2);
}

#[test]
fn missing_source_file() {
    let input = "\
<!-- embed-it src=\"nonexistent.rs\" -->
<!-- /embed-it -->
";
    let result = process_content(input, fixtures_dir());
    // Should leave content unchanged when source file missing.
    assert_eq!(result, input);
}

#[test]
fn missing_closing_tag() {
    let input = "<!-- embed-it src=\"example.rs\" -->\nstale\n";
    let result = process_content(input, fixtures_dir());
    assert_eq!(result, input);
}

#[test]
fn idempotent() {
    let input = "\
<!-- embed-it src=\"example.rs\" fence=\"auto\" -->
<!-- /embed-it -->
";
    let first = process_content(input, fixtures_dir());
    let second = process_content(&first, fixtures_dir());
    assert_eq!(first, second);
}

#[test]
fn unknown_extension_fenced() {
    let input = "\
<!-- embed-it src=\"example.rs\" fence=\"auto\" -->
<!-- /embed-it -->
";
    let result = process_content(input, fixtures_dir());
    assert!(result.contains("```rust"));
}

#[test]
fn process_file_works() {
    let tmp_dir = std::env::temp_dir().join("embed-it-test");
    std::fs::create_dir_all(&tmp_dir).unwrap();

    let src = fixtures_dir().join("example.rs");
    std::fs::copy(&src, tmp_dir.join("example.rs")).unwrap();

    let md_src = fixtures_dir().join("sample.md");
    let md_dst = tmp_dir.join("sample.md");
    std::fs::copy(&md_src, &md_dst).unwrap();

    let result = process_file(&md_dst).unwrap();
    let expected = std::fs::read_to_string(fixtures_dir().join("sample_expected.md")).unwrap();
    assert_eq!(result.processed, expected);
}

#[test]
fn verify_mode_detects_changes() {
    let tmp_dir = std::env::temp_dir().join("embed-it-verify-test");
    std::fs::create_dir_all(&tmp_dir).unwrap();

    std::fs::copy(
        fixtures_dir().join("example.rs"),
        tmp_dir.join("example.rs"),
    )
    .unwrap();
    std::fs::copy(fixtures_dir().join("sample.md"), tmp_dir.join("sample.md")).unwrap();

    let result = process_file(&tmp_dir.join("sample.md")).unwrap();
    assert_ne!(result.original, result.processed);
}

#[test]
fn dry_run_does_not_modify() {
    let tmp_dir = std::env::temp_dir().join("embed-it-dry-test");
    std::fs::create_dir_all(&tmp_dir).unwrap();

    std::fs::copy(
        fixtures_dir().join("example.rs"),
        tmp_dir.join("example.rs"),
    )
    .unwrap();
    let md_dst = tmp_dir.join("sample.md");
    std::fs::copy(fixtures_dir().join("sample.md"), &md_dst).unwrap();

    let original_content = std::fs::read_to_string(&md_dst).unwrap();

    let _result = process_file(&md_dst).unwrap();

    let after = std::fs::read_to_string(&md_dst).unwrap();
    assert_eq!(original_content, after);
}

#[test]
fn nested_fences() {
    let input = "\
<!-- embed-it src=\"has_fences.md\" fence=\"auto\" -->
<!-- /embed-it -->
";
    let result = process_content(input, fixtures_dir());
    assert!(
        result.contains("````"),
        "outer fence should be at least 4 backticks"
    );
    assert!(
        !result.starts_with("```\n"),
        "outer fence should not be exactly 3 backticks"
    );
}

#[test]
fn nested_fences_idempotent() {
    let input = "\
<!-- embed-it src=\"has_fences.md\" fence=\"auto\" -->
<!-- /embed-it -->
";
    let first = process_content(input, fixtures_dir());
    let second = process_content(&first, fixtures_dir());
    assert_eq!(first, second, "dynamic fence should not grow on re-runs");
}

// --- File-based tests for every comment style ---

fn run_host_test(host_file: &str, expected_file: &str) {
    let tmp_dir = std::env::temp_dir().join(format!("embed-it-{}", host_file));
    std::fs::create_dir_all(&tmp_dir).unwrap();

    // Copy source file and host file to temp dir.
    std::fs::copy(
        fixtures_dir().join("example.rs"),
        tmp_dir.join("example.rs"),
    )
    .unwrap();
    let dst = tmp_dir.join(host_file);
    std::fs::copy(fixtures_dir().join(host_file), &dst).unwrap();

    let result = process_file(&dst).unwrap();
    let expected = std::fs::read_to_string(fixtures_dir().join(expected_file)).unwrap();
    assert_eq!(result.processed, expected, "mismatch for {}", host_file);

    // Idempotency: processing the result again should produce the same output.
    std::fs::write(&dst, &result.processed).unwrap();
    let second = process_file(&dst).unwrap();
    assert_eq!(
        second.processed, result.processed,
        "idempotency failed for {}",
        host_file
    );
}

#[test]
fn host_html() {
    run_host_test("host.html", "host_expected.html");
}

#[test]
fn host_rs() {
    run_host_test("host.rs", "host_expected.rs");
}

#[test]
fn host_py() {
    run_host_test("host.py", "host_expected.py");
}

#[test]
fn host_css() {
    run_host_test("host.css", "host_expected.css");
}

#[test]
fn host_sql() {
    run_host_test("host.sql", "host_expected.sql");
}

#[test]
fn host_yml() {
    run_host_test("host.yml", "host_expected.yml");
}

// --- New tests for generalized behavior ---

#[test]
fn raw_insertion() {
    let input = "\
<!-- embed-it src=\"example.rs\" -->
<!-- /embed-it -->
";
    let result = process_content(input, fixtures_dir());
    // Raw: no backtick fences.
    assert!(
        !result.contains("```"),
        "raw insertion should not have fences"
    );
    assert!(result.contains("fn main()"));
    assert!(result.contains("println!"));
}

#[test]
fn fence_explicit_language() {
    let input = "\
<!-- embed-it src=\"example.rs\" fence=\"python\" -->
<!-- /embed-it -->
";
    let result = process_content(input, fixtures_dir());
    assert!(
        result.contains("```python"),
        "should use explicit language tag"
    );
    assert!(result.contains("fn main()"));
}

#[test]
fn fence_bare_attribute() {
    // fence without ="..." should auto-detect
    let input = "\
<!-- embed-it src=\"example.rs\" fence -->
<!-- /embed-it -->
";
    let result = process_content(input, fixtures_dir());
    assert!(
        result.contains("```rust"),
        "bare fence should auto-detect language"
    );
}

#[test]
fn non_markdown_host_file_hash_comments() {
    let input = "\
# Configuration file
# embed-it src=\"example.rs\"
# /embed-it
";
    let result = process_content(input, fixtures_dir());
    assert!(
        result.contains("fn main()"),
        "should embed in # comment files"
    );
    assert!(
        !result.contains("```"),
        "raw insertion by default in non-markdown files"
    );
}

#[test]
fn non_markdown_host_file_slash_comments() {
    let input = "\
// embed-it src=\"example.rs\" fence=\"auto\"
// /embed-it
";
    let result = process_content(input, fixtures_dir());
    assert!(
        result.contains("```rust"),
        "should embed with fences in // comment files"
    );
    assert!(result.contains("fn main()"));
}
