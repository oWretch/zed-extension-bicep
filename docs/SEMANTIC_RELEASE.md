# Semantic Release

This document explains the use of semantic-release for automated versioning and
releases.

## Overview

This project uses [semantic-release](https://semantic-release.gitbook.io/) for
automated version management and release creation. Semantic-release
automatically determines version numbers, generates release notes, and publishes
releases based on commit messages following the
[Conventional Commits](https://www.conventionalcommits.org/) specification.

## How It Works

1. **Commit Analysis**: semantic-release analyzes commit messages to determine
   the type of release needed
2. **Version Calculation**: Based on commit types, it calculates the next
   version number following semver
3. **Release Notes**: Automatically generates release notes from commit messages
4. **File Updates**: Updates `Cargo.toml` and `extension.toml` with new version
5. **Git Operations**: Commits changes, creates tags, and pushes to repository
6. **GitHub Release**: Creates a GitHub release with generated notes (serves as the changelog)

## Commit Message Format

Use [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types and Their Effects

- `feat:` - New feature (triggers minor version bump)
- `fix:` - Bug fix (triggers patch version bump)
- `docs:` - Documentation changes (no version bump)
- `style:` - Code style changes (no version bump)
- `refactor:` - Code refactoring (no version bump)
- `test:` - Test additions/changes (no version bump)
- `chore:` - Maintenance tasks (no version bump)
- `ci:` - CI/CD changes (no version bump)
- `perf:` - Performance improvements (triggers patch version bump)

### Breaking Changes

For major version bumps, include `BREAKING CHANGE:` in the commit footer or use
`!` after the type:

```
feat!: remove deprecated API endpoint

BREAKING CHANGE: The /old-api endpoint has been removed. Use /new-api instead.
```

## Configuration Files

### `.releaserc.json`

Contains the semantic-release configuration including:

- Branch configuration (main)
- Plugin sequence and settings
- File update commands
- Git commit settings

### `package.json`

Defines the Node.js dependencies needed for semantic-release and includes
helpful scripts:

- `npm run release:dry-run` - Test release process without publishing
- `npm run release` - Manual release (normally done by CI)

## GitHub Actions Workflow

The updated workflow (`.github/workflows/release.yaml`):

1. Checks out code with full git history
2. Sets up Node.js environment
3. Installs semantic-release dependencies
4. Runs semantic-release with GitHub token

## Local Development

### Testing Releases Locally

```bash
# Install dependencies
npm install

# Dry run to see what would be released
npm run release:dry-run

# Check if commits follow conventional format
npx commitizen init cz-conventional-changelog --save-dev --save-exact
```

### Manual Release

If needed, you can trigger a release manually:

```bash
npm run release
```

**Note**: This requires a `GITHUB_TOKEN` environment variable with appropriate
permissions.

## Troubleshooting

### No Release Created

- Check that commits follow conventional commit format
- Ensure commits contain releasable changes (feat, fix, perf, or breaking
  changes)
- Verify the GitHub token has sufficient permissions

### Version Update Failures

- Check that `Cargo.toml` and `extension.toml` are writable
- Verify the file update command in `.releaserc.json` works correctly

### Permission Issues

- Ensure the GitHub Actions workflow has `contents: write` and `issues: write`
  permissions
- Verify the `GITHUB_TOKEN` secret is available

## Benefits of Migration

1. **Automated Versioning**: No more manual version management
2. **Consistent Release Notes**: Generated from commit messages
3. **Conventional Commits**: Enforces consistent commit message format
4. **Simplified CI**: Less complex GitHub Actions workflow
5. **GitHub Release Notes**: Automatically generated and serve as the project changelog
6. **Semantic Versioning**: Proper semver based on change types

## Migration Notes

- The initial version (1.1.0) was preserved during migration
- Future releases will be automatically determined by commit messages
- All previous manual versioning processes should be avoided
- Contributors should follow conventional commit guidelines
