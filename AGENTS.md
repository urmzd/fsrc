# AGENTS.md

## Identity

You are an agent working on **embed-src** — a Rust CLI tool that embeds source files into any text file using comment markers. It works with markdown, YAML, Python, Rust, shell scripts, and any file that supports comments.

## Architecture

Single Rust binary (not a workspace). Minimal dependencies: `clap` and `regex`.

| File | Role |
|------|------|
| `src/main.rs` | CLI entry point (`clap` derive) — `--verify`, `--dry-run` flags |
| `src/embed.rs` | Core processing — finds `embed-src src="..."` markers, reads source files, replaces content |
| `src/lang.rs` | `ext_to_lang()` — maps file extensions to code fence language identifiers |
| `src/lib.rs` | Library re-exports |
| `action.yml` | GitHub Action wrapper |

## How It Works

1. Scans for lines containing `embed-src src="path"` (opening marker)
2. Finds the corresponding `/embed-src` (closing marker)
3. Replaces content between markers with the referenced file's contents
4. Supports raw insertion (default) or code-fenced insertion (`fence`, `fence="auto"`, `fence="python"`)
5. Idempotent — safe to re-run

## Commands

| Task | Command |
|------|---------|
| Build | `cargo build --release` |
| Test | `cargo test` |
| Install | `cargo install --path .` |
| Process files | `embed-src README.md docs/*.md` |
| Verify (CI) | `embed-src --verify README.md` |
| Dry run | `embed-src --dry-run README.md` |

## Code Style

- Rust 2021 edition, Apache-2.0 license
- Regex-based marker parsing (comment-agnostic)
- Backtick fence collision avoidance (`make_fence()`)
- Tests inline in `embed.rs` and `lang.rs`

## Adding Language Support

Edit `src/lang.rs` → `ext_to_lang()` match arms. Add the file extension mapping and a test in the `tests` module below.
