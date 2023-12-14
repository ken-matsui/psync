# psync [![crates.io version](https://img.shields.io/crates/v/psync.svg)](https://crates.io/crates/psync) [![crates.io downloads](https://img.shields.io/crates/d/psync.svg)](https://crates.io/crates/psync)

A tool to sync software across devices

## Installation

```sh
cargo install psync
```

## Usage

### `~/.config/psync/config.toml`

```toml
[homebrew]
taps = [
    "hashicorp/tap",
    { name = "messense/macos-cross-toolchains", on = "macos" },
]
forumale = [
    "neovim",
    { name = "trash", on = "macos" },
]
# You do not need to specify `on` for `casks` since casks are only for macOS
casks = [
    "alacritty",
    "discord",
]

[cargo]
crates = [
    "cargo-update",
    "suggest-command-not-found",
]

[snap]
snaps = [
    { name = "alacritty", classic = true },
    "discord",
]
```

When you run `psync`, it automatically installs only specified, not-installed packages.

A full example is placed [here](https://github.com/ken-matsui/dotfiles/blob/main/.config/psync/config.toml).

## Contribution

Contributions, including issues and pull requests, are very welcome.

### Build

```bash
$ cargo build
```

### Test

```bash
$ cargo test
```

### Publish

#### [GitHub Releases](https://github.com/ken-matsui/keep-installed/tags)

```bash
$ git tag v0.1.0
$ git push origin v0.1.0
```

#### [crates.io](https://crates.io/)

```bash
$ cargo publish
```
