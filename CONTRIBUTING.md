# Contributing to mcp-sys

Thanks for your interest in contributing! 🦀

## How to contribute

1. Fork the repo
2. Create a branch from `dev`: `git checkout -b feat/your-feature`
3. Make your changes
4. Run `cargo test` and make sure everything passes
5. Open a Pull Request against `dev` (not `main`)

## Reporting bugs

Use the [Bug Report](.github/ISSUE_TEMPLATE/bug_report.md) template.

## Suggesting features

Use the [Feature Request](.github/ISSUE_TEMPLATE/feature_request.md) template.

## Code style

- Follow standard Rust conventions
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings

## Branch strategy

| Branch | Purpose |
|---|---|
| `main` | Stable releases only |
| `dev` | Active development |
| `feat/*` | New features |
| `fix/*` | Bug fixes |