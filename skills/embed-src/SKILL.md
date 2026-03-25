---
name: embed-src
description: Embed source files into documents using comment markers. Use when syncing code snippets into README, docs, or any text file that references external source files.
metadata:
  argument-hint: [files...]
---

# Embed Source Files

Embed source files into documents using `embed-src`.

## Steps

1. Ensure the target file has proper markers:
   - Opening: `<!-- embed-src src="path/to/file" -->` (or any comment style)
   - Closing: `<!-- /embed-src -->`
   - Optional: add `fence` or `fence="auto"` for code-fenced output
2. Run: `embed-src $ARGUMENTS` (or `embed-src README.md` if no args)
3. To check if files are up-to-date (CI): `embed-src --verify <files>`
4. To preview changes: `embed-src --dry-run <files>`

## Marker Syntax

Works with any comment style:

```
<!-- embed-src src="config.yml" fence="auto" -->   (HTML/Markdown)
// embed-src src="utils.py"                         (Rust/JS/Go)
# embed-src src="setup.sh"                          (Python/Shell/YAML)
-- embed-src src="schema.sql"                        (SQL/Lua)
```

## Fence Options

| Attribute | Behavior |
|-----------|----------|
| *(none)* | Raw insertion |
| `fence` | Code fence with auto-detected language |
| `fence="auto"` | Code fence with auto-detected language |
| `fence="python"` | Code fence with explicit language tag |
