# Embed MD

Automate embedding code into markdown files with this GitHub Action. Uses a lightweight Rust CLI to replace tagged regions with source file contents.

## Syntax

In your markdown files, use the following directive pair:

```markdown
<!-- embedmd src="path/to/file.rs" -->
<!-- /embedmd -->
```

When the action runs, the content between the tags is replaced with a fenced code block containing the referenced file:

````markdown
<!-- embedmd src="path/to/file.rs" -->
```rust
fn main() {
    println!("Hello");
}
```
<!-- /embedmd -->
````

- Paths are relative to the markdown file's directory.
- The code fence language is inferred from the file extension.
- Re-running is idempotent -- existing content between tags is replaced.

## Features

- **Automatic Embedding**: Embed code snippets from source files into markdown.
- **Custom Commit Options**: Personalize commit messages, author details, and push behavior.
- **Dry-Run Mode**: Test embedding without creating commits.
- **Seamless Integration**: Drop into any GitHub Actions workflow.

## Inputs

| Name | Description | Required | Default |
|------|-------------|----------|---------|
| `markdown-files` | Space-separated list of markdown files to process. | No | `README.md` |
| `commit-message` | Commit message for the embedded changes. | No | `chore: embed code within markdown files` |
| `commit-name` | Git committer name. | No | `github-actions[bot]` |
| `commit-email` | Git committer email. | No | `github-actions[bot]@users.noreply.github.com` |
| `commit-push` | Whether to push after committing. | No | `true` |
| `commit-dry` | Skip the commit (dry-run mode). | No | `false` |
| `github-token` | GitHub token for downloading the binary. | No | `${{ github.token }}` |

## Usage

### Basic

<!-- embedmd src="examples/example.yml" -->
```yaml
name: "Example"

on: [push]

jobs:
  embed:
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - name: "Checkout repo"
        uses: actions/checkout@v4
      - name: "Embed code into markdown"
        uses: urmzd/embed-md@v1.4.0
        with:
          markdown-files: "README.md"
```
<!-- /embedmd -->

### Multiple Files

```yaml
- uses: urmzd/embed-md@v1.4.0
  with:
    markdown-files: "README.md docs/API.md docs/GUIDE.md"
```

### Dry Run (No Commit)

Useful for CI validation -- embed the files and check for drift without committing:

```yaml
- uses: urmzd/embed-md@v1.4.0
  with:
    commit-dry: "true"
    commit-push: "false"
```

### CLI Usage

The `embed-md` binary can also be used directly:

```bash
# Install from GitHub releases
curl -sSfL https://github.com/urmzd/embed-md/releases/latest/download/embed-md-x86_64-unknown-linux-gnu -o /usr/local/bin/embed-md
chmod +x /usr/local/bin/embed-md

# Process files in place
embed-md README.md docs/*.md

# Check if files are up-to-date (CI mode)
embed-md --verify README.md

# Preview changes without writing
embed-md --dry-run README.md
```

## Troubleshooting

**Action fails with "nothing to commit"**
This means no changes were needed. Ensure your markdown files contain valid `<!-- embedmd src="..." -->` directives with corresponding `<!-- /embedmd -->` closing tags.

**Permission denied on push**
The action needs `contents: write` permission. Add this to your job:
```yaml
permissions:
  contents: write
```

**Files not being embedded**
Verify the file paths in `markdown-files` are relative to the repository root and that the referenced source files exist.

## Internal Use

We use Embed MD in our own CI/CD pipelines, ensuring our documentation is always synchronized with the latest code.
