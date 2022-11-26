# EmbedMd

Embed code directly into your markdown files with ease.

## Usage

```yaml
# .github/<action>.yml
uses: actions/embedme-action@v1
with:
    # defaults
    markdown-files: 
        - README.md
    commit-message: "chore: embed code within README"
```
