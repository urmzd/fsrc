This is an example markdown file.

<!-- embedmd src="example.yml" -->
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
