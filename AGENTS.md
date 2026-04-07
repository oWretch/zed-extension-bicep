# AGENTS.md — Coding Agent Guide

## Project Overview

This is a **Zed editor extension** that adds [Bicep](https://docs.microsoft.com/azure/azure-resource-manager/bicep/) language support — syntax highlighting, IntelliSense, and error checking for both `.bicep` and `.bicepparam` files.

The extension is written in **Rust**, compiled to **WebAssembly** (`wasm32-wasip2`), and uses the [`zed_extension_api`](https://docs.rs/zed_extension_api) crate. At runtime it downloads the official [Bicep Language Server](https://github.com/Azure/bicep) and launches it via the .NET runtime.

## Architecture

```
zed-extension-bicep/
├── src/
│   └── bicep.rs              # Extension entry point (Rust → WASM)
├── extension.toml             # Zed extension metadata (name, version, grammars, LSP config)
├── Cargo.toml                 # Rust package manifest (version must match extension.toml)
├── languages/
│   ├── bicep/                 # Language config for .bicep files
│   │   ├── config.toml        #   File associations, brackets, comments
│   │   ├── highlights.scm     #   Syntax highlighting queries (tree-sitter)
│   │   ├── indents.scm        #   Auto-indentation rules
│   │   ├── injections.scm     #   Language injection queries
│   │   ├── outline.scm        #   Symbol outline queries
│   │   └── brackets.scm       #   Bracket matching queries
│   └── bicep_params/          # Language config for .bicepparam files (same structure)
├── grammars/
│   ├── bicep/                 # Tree-sitter grammar for Bicep (DO NOT HAND-EDIT src/)
│   └── bicep_params/          # Tree-sitter grammar for Bicep Params (DO NOT HAND-EDIT src/)
├── .github/
│   └── workflows/
│       ├── release.yaml       # Automated release via semantic-release
│       └── ci.yaml            # PR checks (fmt, clippy, build)
├── .releaserc.json            # semantic-release configuration
├── package.json               # Node.js deps for semantic-release
├── scripts/
│   └── test-version-update.js # Validates version bump logic
├── docs/
│   └── SEMANTIC_RELEASE.md    # Release process documentation
├── test.bicep                 # Sample Bicep file for manual testing
├── test.bicepparam            # Sample Bicep params file for manual testing
└── bicepconfig.json           # Bicep configuration for test files
```

## Key Concepts

### Extension Entry Point (`src/bicep.rs`)

The single Rust source file implements the `zed::Extension` trait:

1. **`language_server_command()`** — Called by Zed to get the command to launch the LSP. Returns `dotnet --roll-forward Major <path-to-Bicep.LangServer.dll>`.
2. **`dotnet_binary_path()`** — Resolves and caches the `dotnet` binary location from the system PATH.
3. **`language_server_path()`** — Downloads the latest Bicep language server release from GitHub (`Azure/bicep`), extracts `bicep-langserver.zip`, and cleans up old versions.

### Version Management

Versions in `Cargo.toml` and `extension.toml` **must always match**. They are updated automatically by semantic-release — **never bump versions manually**.

### Tree-sitter Grammars

The `grammars/` directory contains vendored tree-sitter grammars:

- **`grammars/*/src/`** — **GENERATED FILES. DO NOT EDIT.** These are produced by `tree-sitter generate` from `grammar.js`.
- **`grammars/*/grammar.js`** — The grammar definition. Edit this to change parsing rules.
- **`grammars/*/test/corpus/`** — Tree-sitter test cases.

The upstream grammar repos are referenced by commit hash in `extension.toml` under `[grammars.bicep]` and `[grammars.bicep_params]`.

### Language Queries (`languages/`)

Tree-sitter query files (`.scm`) use S-expression syntax. These are hand-maintained and define how Zed interprets parsed syntax trees:

- **`highlights.scm`** — Maps tree-sitter nodes to highlight groups (`@keyword`, `@function.call`, `@variable`, etc.)
- **`indents.scm`** — Rules for auto-indentation (`@indent.begin`, `@indent.end`, `@indent.branch`)
- **`injections.scm`** — Language injection (currently injects `comment` language into comment nodes)
- **`outline.scm`** — Symbol outline for the sidebar (`@item`, `@name`, `@context`)
- **`brackets.scm`** — Bracket pair definitions (`@open`, `@close`)
- **`config.toml`** — Language metadata: file extensions, comment styles, bracket definitions, tab size

## Build & Test

### Prerequisites

- **Rust** toolchain with `wasm32-wasip2` target: `rustup target add wasm32-wasip2`
- **Node.js** (for semantic-release tooling): `npm install`

### Commands

```bash
# Build the WASM extension
cargo build --target wasm32-wasip2

# Check formatting
cargo fmt --check

# Lint
cargo clippy --target wasm32-wasip2 -- -D warnings

# Run repository tests
npm test

# Test tree-sitter grammars (requires: npm install -g tree-sitter-cli)
cd grammars/bicep && tree-sitter test
cd grammars/bicep_params && tree-sitter test

# Dry-run a release locally
npm run release:dry-run-local
```

### No Unit Tests

The Rust code currently has no unit tests. `npm test` validates the semantic-release version bump logic and smoke-tests representative `.bicep` and `.bicepparam` fixtures against the pinned tree-sitter grammar commits from `extension.toml`.

### Grammar Testing

This repository smoke-tests the pinned upstream grammar revisions by parsing the files under `fixtures/grammar/` with the exact commits listed in `extension.toml`.

To test locally:

```bash
npm install
npm run test:grammars
```

The deeper corpus tests still live in the upstream grammar repositories. For parser changes, add or update corpus coverage there as well.

## Files to Never Edit

- **`grammars/*/src/`** — Generated by tree-sitter. Edit `grammar.js` in the upstream repos instead.
- **`Cargo.toml` version / `extension.toml` version** — Managed by semantic-release. Never bump manually.

## Conventions

### Commits

This project uses [Conventional Commits](https://www.conventionalcommits.org/). Every commit message must follow the format:

```
<type>[optional scope]: <description>
```

Types and version effects:

- `feat:` — new feature (minor bump)
- `fix:` — bug fix (patch bump)
- `perf:` — performance improvement (patch bump)
- `docs:` — documentation only (no bump)
- `style:` — code style/formatting (no bump)
- `refactor:` — code refactoring (no bump)
- `test:` — test changes (no bump)
- `chore:` — maintenance (no bump)
- `ci:` — CI/CD changes (no bump)

See [docs/SEMANTIC_RELEASE.md](docs/SEMANTIC_RELEASE.md) for full details.

### Rust Style

- Edition 2021, formatted with `rustfmt` (config in `rustfmt.toml`)
- Target: `wasm32-wasip2` (not native)
- Single crate, single source file (`src/bicep.rs`)
- Uses `zed_extension_api` types (aliased as `zed`) — do not use `std::net`, `std::thread`, or other APIs unavailable in WASI

### Tree-sitter Queries

Files in `languages/*/` use S-expression syntax (`.scm` files). Common highlight groups: `@keyword`, `@function.call`, `@variable`, `@type`, `@string`, `@number`, `@comment`, `@property`, `@attribute`, `@operator`, `@punctuation.bracket`, `@punctuation.delimiter`.

Reference `grammars/*/src/node-types.json` for available node types when writing queries.

### Pre-commit Hooks

Install pre-commit hooks after cloning:

```bash
pre-commit install
pre-commit install --hook-type commit-msg
```

This enforces formatting, linting, and conventional commit messages before each commit.

### Testing Checklist

Before submitting changes, all of these must pass:

- `cargo build --target wasm32-wasip2` — must compile
- `cargo fmt --check` — must pass
- `cargo clippy --target wasm32-wasip2 -- -D warnings` — must pass
- `npm test` — validates version bump logic and grammar fixtures

## Common Agent Tasks

### Modifying LSP Download/Launch Logic

Edit `src/bicep.rs`. The key methods are on the `BicepExtension` impl block. After changes, verify with:

```bash
cargo build --target wasm32-wasip2
cargo clippy --target wasm32-wasip2 -- -D warnings
```

### Updating Syntax Highlighting

Edit `languages/bicep/highlights.scm` (or `languages/bicep_params/highlights.scm`). These use tree-sitter query syntax. Reference the grammar's `node-types.json` for available node types:

- `grammars/bicep/src/node-types.json`
- `grammars/bicep_params/src/node-types.json`

### Adding Outline Support for New Constructs

Edit `languages/bicep/outline.scm`. Each outline entry needs `@item` (the whole node), `@name` (the identifier), and `@context` (the keyword like `param`, `resource`, etc.).

### Updating Tree-sitter Grammar Commit

In `extension.toml`, update the `commit` field under `[grammars.bicep]` or `[grammars.bicep_params]` to point to a new commit in the upstream grammar repo. Then rebuild — Zed fetches and builds the grammar from the specified commit.

Grammar source repos:
- [tree-sitter-bicep](https://github.com/oWretch/tree-sitter-bicep)
- [tree-sitter-bicep-params](https://github.com/oWretch/tree-sitter-bicep-params)

After updating the commit hash in `extension.toml`, run `npm run test:grammars` to validate the pinned revisions against this repository's fixture files. Parser corpus tests still belong in the upstream grammar repositories.

### Adding a New Language Configuration Option

Edit `languages/bicep/config.toml` (or `languages/bicep_params/config.toml`). See [Zed language configuration docs](https://zed.dev/docs/extensions/languages) for available options.
