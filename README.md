# git-account-switcher

Declarative Git & GitHub CLI (`gh`) multi-account manager.

## Overview

Manage multiple Git accounts and GitHub credentials seamlessly based on directory structure.

- **Single source of truth**: Declare accounts in `accounts.toml`.
- **Zero pollution**: Outputs generated files to `~/.config/gh-multiaccount/` without modifying user-owned `~/.gitconfig` or `~/.ssh/config`.
- **Fast shell integration**: Works with `direnv` or lightweight shell function wrappers.
- **Diagnostics**: `doctor` command detects configuration drift, missing includes, and permission issues.

## Installation

### Shell installer (macOS & Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/n-someya/git-account-switcher/releases/latest/download/git-account-switcher-installer.sh | sh
```

### Via npx

```bash
npx git-account-switcher --help
```

## Quick Start

```bash
# 1. Initialize accounts.toml by inspecting existing setup
git-account-switcher init

# 2. Review and edit ~/.config/gh-multiaccount/accounts.toml

# 3. Generate configuration files
git-account-switcher apply

# 4. Check setup health
git-account-switcher doctor
```

## Development

- [docs/toolchain.md](docs/toolchain.md): Development environment and installed toolchain versions.
- [docs/dependencies.md](docs/dependencies.md): Package and library reference with usage examples.
- [docs/cicd.md](docs/cicd.md): CI/CD pipeline architecture and release instructions.

## License



MIT OR Apache-2.0

