# Embed It

Embed source files into **any text file** using comment markers. Works with markdown, YAML, Python, Rust, shell scripts, and any file that supports comments.

## Syntax

Place opening and closing markers in your file using whatever comment style is appropriate:

**Markdown / HTML:**
```markdown
<!-- embed-it src="path/to/file.rs" -->
<!-- /embed-it -->
```

**Rust / JS / Go / C:**
```rust
// embed-it src="path/to/file.rs"
// /embed-it
```

**Python / Shell / YAML:**
```python
# embed-it src="path/to/file.rs"
# /embed-it
```

**CSS:**
```css
/* embed-it src="path/to/file.rs" */
/* /embed-it */
```

**SQL / Lua:**
```sql
-- embed-it src="path/to/file.rs"
-- /embed-it
```

When the tool runs, the content between the markers is replaced with the referenced file's contents.

### Raw vs Fenced Insertion

By default, content is inserted **raw** (no wrapping). This works for any file type.

To wrap content in markdown code fences, use the `fence` attribute:

| Attribute | Behavior |
|-----------|----------|
| *(none)* | Raw insertion |
| `fence` | Code fence with auto-detected language |
| `fence="auto"` | Code fence with auto-detected language |
| `fence="python"` | Code fence with explicit language tag |

**Example with fencing:**

````markdown
<!-- embed-it src="path/to/file.rs" fence="auto" -->
```rust
fn main() {
    println!("Hello");
}
```
<!-- /embed-it -->
````

- Paths are relative to the host file's directory.
- The code fence language is inferred from the file extension when using `fence` or `fence="auto"`.
- Re-running is idempotent -- existing content between markers is replaced.

## Features

- **Any File Type**: Embed into markdown, YAML, Python, Rust, or any file with comments.
- **Raw or Fenced**: Insert raw content by default, or wrap in code fences with `fence`.
- **Custom Commit Options**: Personalize commit messages, author details, and push behavior.
- **Dry-Run Mode**: Test embedding without creating commits.
- **Seamless Integration**: Drop into any GitHub Actions workflow.

## Inputs

| Name | Description | Required | Default |
|------|-------------|----------|---------|
| `files` | Space-separated list of files to process. | No | `README.md` |
| `commit-message` | Commit message for the embedded changes. | No | `chore: embed source files` |
| `commit-name` | Git committer name. | No | `github-actions[bot]` |
| `commit-email` | Git committer email. | No | `github-actions[bot]@users.noreply.github.com` |
| `commit-push` | Whether to push after committing. | No | `true` |
| `commit-dry` | Skip the commit (dry-run mode). | No | `false` |
| `github-token` | GitHub token for downloading the binary. | No | `${{ github.token }}` |

## Usage

### Basic

<!-- embed-it src="examples/example.yml" fence="auto" -->
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
      - name: "Embed code into files"
        uses: urmzd/embed-it@v1.4.0
        with:
          files: "README.md"
```
<!-- /embed-it -->

### Multiple Files

```yaml
- uses: urmzd/embed-it@v1.4.0
  with:
    files: "README.md docs/API.md docs/GUIDE.md"
```

### Dry Run (No Commit)

Useful for CI validation -- embed the files and check for drift without committing:

```yaml
- uses: urmzd/embed-it@v1.4.0
  with:
    commit-dry: "true"
    commit-push: "false"
```

### CLI Usage

The `embed-it` binary can also be used directly:

```bash
# Process files in place
embed-it README.md docs/*.md

# Check if files are up-to-date (CI mode)
embed-it --verify README.md

# Preview changes without writing
embed-it --dry-run README.md
```

## Troubleshooting

**Action fails with "nothing to commit"**
This means no changes were needed. Ensure your files contain valid `embed-it src="..."` opening markers with corresponding `/embed-it` closing markers.

**Permission denied on push**
The action needs `contents: write` permission. Add this to your job:
```yaml
permissions:
  contents: write
```

**Files not being embedded**
Verify the file paths in `files` are relative to the repository root and that the referenced source files exist.

## Internal Use

We use Embed It in our own CI/CD pipelines, ensuring our documentation is always synchronized with the latest code.
