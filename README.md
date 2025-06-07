# Bicep Extension for Zed

This extension adds support for
[Bicep](https://docs.microsoft.com/azure/azure-resource-manager/bicep/) to
[Zed](https://zed.dev/). This includes syntax highlighting, IntelliSense, and
error checking for both Bicep and Bicep Parameter files.

## Pre-requisites

The Bicep language server is built in .NET, so requires the .NET runtime to be
installed. [.Net 8.0](https://dotnet.microsoft.com/en-us/download/dotnet/8.0) or
later is required. 8.0 is recommended as the latest LTS version.

## Contributing

This project uses [semantic-release](https://semantic-release.gitbook.io/) for automated versioning and releases. Please follow [Conventional Commits](https://www.conventionalcommits.org/) when making contributions.

### Commit Message Format

- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `chore:` for maintenance tasks
- `ci:` for CI/CD changes

### Development

To work on this extension:

1. Clone the repository
2. Make your changes
3. Commit using conventional commit messages
4. Create a pull request

Releases are automatically created when changes are merged to the `main` branch.
