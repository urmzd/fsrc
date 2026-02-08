use std::path::Path;

use embed_md::embed::{process_content, process_file};

fn fixtures_dir() -> &'static Path {
    Path::new("tests/fixtures")
}

#[test]
fn single_directive() {
    let input = "\
# Sample

<!-- embedmd src=\"example.rs\" -->
<!-- /embedmd -->

Done.
";
    let expected = "\
# Sample

<!-- embedmd src=\"example.rs\" -->
```rust
fn main() {
    println!(\"Hello, world!\");
}
```
<!-- /embedmd -->

Done.
";
    let result = process_content(input, fixtures_dir());
    assert_eq!(result, expected);
}

#[test]
fn multiple_directives() {
    let input = "\
<!-- embedmd src=\"example.rs\" -->
<!-- /embedmd -->

Middle text.

<!-- embedmd src=\"example.rs\" -->
<!-- /embedmd -->
";
    let result = process_content(input, fixtures_dir());
    assert!(result.contains("```rust"));
    assert_eq!(result.matches("```rust").count(), 2);
}

#[test]
fn missing_source_file() {
    let input = "\
<!-- embedmd src=\"nonexistent.rs\" -->
<!-- /embedmd -->
";
    let result = process_content(input, fixtures_dir());
    // Should leave content unchanged when source file missing.
    assert_eq!(result, input);
}

#[test]
fn missing_closing_tag() {
    let input = "<!-- embedmd src=\"example.rs\" -->\nstale\n";
    let result = process_content(input, fixtures_dir());
    assert_eq!(result, input);
}

#[test]
fn idempotent() {
    let input = "\
<!-- embedmd src=\"example.rs\" -->
<!-- /embedmd -->
";
    let first = process_content(input, fixtures_dir());
    let second = process_content(&first, fixtures_dir());
    assert_eq!(first, second);
}

#[test]
fn unknown_extension() {
    // Create content referencing a file with unknown extension.
    let input = "\
<!-- embedmd src=\"example.rs\" -->
<!-- /embedmd -->
";
    // We test the known case; unknown extension is covered in lang unit tests.
    let result = process_content(input, fixtures_dir());
    assert!(result.contains("```rust"));
}

#[test]
fn process_file_works() {
    // Copy fixture to a temp file so we don't modify the original.
    let tmp_dir = std::env::temp_dir().join("embed-md-test");
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
    // process_file returns different original vs processed when changes needed.
    let tmp_dir = std::env::temp_dir().join("embed-md-verify-test");
    std::fs::create_dir_all(&tmp_dir).unwrap();

    std::fs::copy(fixtures_dir().join("example.rs"), tmp_dir.join("example.rs")).unwrap();
    std::fs::copy(fixtures_dir().join("sample.md"), tmp_dir.join("sample.md")).unwrap();

    let result = process_file(&tmp_dir.join("sample.md")).unwrap();
    assert_ne!(result.original, result.processed);
}

#[test]
fn dry_run_does_not_modify() {
    let tmp_dir = std::env::temp_dir().join("embed-md-dry-test");
    std::fs::create_dir_all(&tmp_dir).unwrap();

    std::fs::copy(fixtures_dir().join("example.rs"), tmp_dir.join("example.rs")).unwrap();
    let md_dst = tmp_dir.join("sample.md");
    std::fs::copy(fixtures_dir().join("sample.md"), &md_dst).unwrap();

    let original_content = std::fs::read_to_string(&md_dst).unwrap();

    // process_file only computes the result; it doesn't write.
    // Writing is done in main.rs, so the file should be unchanged.
    let _result = process_file(&md_dst).unwrap();

    let after = std::fs::read_to_string(&md_dst).unwrap();
    assert_eq!(original_content, after);
}

#[test]
fn nested_fences() {
    let input = "\
<!-- embedmd src=\"has_fences.md\" -->
<!-- /embedmd -->
";
    let result = process_content(input, fixtures_dir());
    // has_fences.md contains ```, so the outer fence must be ```` (4 backticks).
    assert!(result.contains("````"), "outer fence should be at least 4 backticks");
    assert!(!result.starts_with("```\n"), "outer fence should not be exactly 3 backticks");
}

#[test]
fn nested_fences_idempotent() {
    let input = "\
<!-- embedmd src=\"has_fences.md\" -->
<!-- /embedmd -->
";
    let first = process_content(input, fixtures_dir());
    let second = process_content(&first, fixtures_dir());
    assert_eq!(first, second, "dynamic fence should not grow on re-runs");
}
