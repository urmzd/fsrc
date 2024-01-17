This is an example markdown file.

```yaml
# example.yml

name: "Example"

on: [release]

jobs:
  embed-example:
    runs-on: ubuntu-latest
    steps:
      - name: "Checkout current repo"
        uses: actions/checkout@v3
      - name: "Run entrypoint script."
        uses: urmzd/embed-md@v1.3.4
        id: "embed-code"
        with:
          markdown-files: "README.md"
          commit-message: "chore: embed example using self"
          commit-name: Urmzd Mukhammadnaim
          commit-email: <urmzd@noreply.com>
          commit-push: "false"

```
