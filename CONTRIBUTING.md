# Contributing

## Prerequisites

- Rust (stable toolchain)
- A `GH_TOKEN` with repo access (for releases)

## Getting Started

```bash
git clone https://github.com/urmzd/fsrc.git
cd fsrc
cargo build
```

## Development

```bash
cargo build          # compile
cargo test           # run tests
cargo fmt            # format code
cargo clippy         # lint
```

## Commit Convention

We use [Angular Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `ci`, `perf`

Use [Conventional Commits](https://www.conventionalcommits.org/) format.

## Pull Requests

1. Fork the repository
2. Create a feature branch (`feat/my-feature`)
3. Make changes and commit using conventional commits
4. Open a pull request against `main`

## Code Style

- `cargo fmt` for formatting
- `clippy` for linting
- No `unsafe` code
