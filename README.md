# Bicep Extension for Zed

[![CI](https://github.com/oWretch/zed-extension-bicep/actions/workflows/ci.yaml/badge.svg)](https://github.com/oWretch/zed-extension-bicep/actions/workflows/ci.yaml)
[![Release](https://github.com/oWretch/zed-extension-bicep/actions/workflows/release.yaml/badge.svg)](https://github.com/oWretch/zed-extension-bicep/actions/workflows/release.yaml)

This extension adds support for
[Bicep](https://docs.microsoft.com/azure/azure-resource-manager/bicep/) to
[Zed](https://zed.dev/). This includes syntax highlighting, IntelliSense, and
error checking for both Bicep and Bicep Parameter files.

## Grammar validation fixtures

The repository includes fixture files under `fixtures/grammar/` that cover
representative `.bicep` and `.bicepparam` syntax, including language features
that are pinned from the upstream tree-sitter grammar repositories. `npm test`
runs smoke tests that check those fixtures parse cleanly and that the Zed query
files under `languages/` compile against the exact grammar commits referenced in
`extension.toml`.

## Pre-requisites

The Bicep language server is built in .NET, so requires the .NET runtime to be
installed. [.NET 8.0](https://dotnet.microsoft.com/en-us/download/dotnet/8.0) or
later is required. 8.0 is recommended as the latest LTS version.

## Architecture

See [AGENTS.md](AGENTS.md) for a complete guide to the project structure,
build system, and conventions. This file also serves as the reference for
coding agents working with this repository.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, commit
conventions, and contribution guidelines.
