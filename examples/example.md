This is an example markdown file.

<!-- embed-it src="example.yml" fence="auto" -->
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
