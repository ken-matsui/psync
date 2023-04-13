# brewp

A tool to sync Homebrew packages via Zsh.

## Installation

You can install `brewp` using [`zplug`](https://github.com/zplug/zplug):

```zsh
zplug "ken-matsui/brewp", as:command
```

Or through [`zinit`](https://github.com/zdharma-continuum/zinit):

```zsh
zinit ice as"command"
zinit light ken-matsui/brewp
```

## Usage

### `~/.config/brewp/config.zsh`

You should have a config file named `$XDG_CONFIG_HOME/brewp/config.zsh` or `~/.config/brewp/config.zsh`.

```zsh
# Specify normal pacakages
brewp asciinema

# Specify taps
brewp homebrew/cask-fonts --tap

# Specify cask pacakages
brewp alacritty --cask
```

When you run `brewp sync`, `brewp` automatically installs only specified, not-installed packages.

A full example is placed [here](https://github.com/ken-matsui/dotfiles/blob/main/.config/brewp/config.zsh).
