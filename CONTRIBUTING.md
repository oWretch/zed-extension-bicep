# Contributing

Thanks for your interest in contributing to the Bicep extension for Zed!

## Prerequisites

- **Rust** toolchain: install via [rustup](https://rustup.rs/)
- **WASM target**: `rustup target add wasm32-wasip2`
- **Node.js** (22+): for semantic-release tooling
- **pre-commit**: `pip install pre-commit` or `brew install pre-commit`

## Getting Started

```bash
# Clone the repository
git clone https://github.com/oWretch/zed-extension-bicep.git
cd zed-extension-bicep

# Install Node.js dependencies (for semantic-release)
npm install

# Install pre-commit hooks
pre-commit install
pre-commit install --hook-type commit-msg

# Build the extension
cargo build --target wasm32-wasip2

# Run checks
cargo fmt --check
cargo clippy --target wasm32-wasip2 -- -D warnings
npm test
```

## Development Workflow

1. Create a feature branch from `main`
2. Make your changes
3. Ensure all checks pass (pre-commit hooks will enforce this)
4. Commit using [Conventional Commits](https://www.conventionalcommits.org/) format
5. Open a pull request against `main`

## Commit Message Format

This project uses [Conventional Commits](https://www.conventionalcommits.org/) with [semantic-release](https://semantic-release.gitbook.io/) for automated versioning. Every commit message must follow:

```
<type>[optional scope]: <description>

[optional body]
```

**Types:**

| Type | Description | Version Bump |
|------|-------------|--------------|
| `feat` | New feature | Minor |
| `fix` | Bug fix | Patch |
| `perf` | Performance improvement | Patch |
| `docs` | Documentation only | None |
| `style` | Code style (formatting) | None |
| `refactor` | Code refactoring | None |
| `test` | Test changes | None |
| `chore` | Maintenance | None |
| `ci` | CI/CD changes | None |

For breaking changes, add `!` after the type or include `BREAKING CHANGE:` in the footer.

See [docs/SEMANTIC_RELEASE.md](docs/SEMANTIC_RELEASE.md) for full release process details.

## Project Structure

See [AGENTS.md](AGENTS.md) for a complete architecture guide covering file layout, build system, and common tasks.

## What to Contribute

- **Syntax highlighting improvements** — Edit `languages/bicep/highlights.scm` or `languages/bicep_params/highlights.scm`
- **LSP integration fixes** — Edit `src/bicep.rs`
- **Outline/indentation improvements** — Edit the relevant `.scm` files in `languages/`
- **Documentation** — Always welcome

## Continuous Integration

Pull requests automatically trigger CI checks:

- **Rust formatting & linting** — `cargo fmt --check` and `cargo clippy`
- **WASM build** — `cargo build --target wasm32-wasip2`
- **Grammar validation** — Tree-sitter corpus tests for Bicep and Bicep Params grammars
- **Version update logic** — `npm test` (validates semantic-release version bump)
- **Conventional commits** — PR title must follow format (e.g., `feat:`, `fix:`)

All checks must pass before merge. No secrets are exposed during PR ci runs — the workflow uses strict read-only permissions (`contents: read`).

## Grammar Testing

If you modify the tree-sitter grammars ([`grammars/bicep/grammar.js`](grammars/bicep/grammar.js) or [`grammars/bicep_params/grammar.js`](grammars/bicep_params/grammar.js)), test locally:

```bash
# Install tree-sitter-cli
npm install -g tree-sitter-cli

# Test the grammar against its corpus
cd grammars/bicep && tree-sitter test
cd grammars/bicep_params && tree-sitter test
```

Note: Grammar changes should be made in the upstream repos and then referenced by commit hash in `extension.toml`.

## Version Management

Versions in `Cargo.toml` and `extension.toml` are managed by semantic-release. **Do not bump versions manually** — they are updated automatically when changes are merged to `main`.

## Tree-sitter Grammars

The grammars in `grammars/` are vendored from upstream repositories:

- [tree-sitter-bicep](https://github.com/oWretch/tree-sitter-bicep)
- [tree-sitter-bicep-params](https://github.com/oWretch/tree-sitter-bicep-params)

To update a grammar, change the `commit` hash in `extension.toml` under the relevant `[grammars.*]` section. Do not edit files under `grammars/*/src/` — they are generated.

## Code of Conduct

Be respectful and constructive. We're all here to make Azure tooling better.
