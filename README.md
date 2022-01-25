# brewp

A useful tool for managing homebrew packages.

## Installation

You can install `brewp` using [`zplug`](https://github.com/zplug/zplug):

```zsh
zplug "ken-matsui/brewp", as:command
```

## Usage

`brewp` will automatically install homebrew packages when the shell startup by writing like the following script in your `.zshrc`:

```zsh
# Specify normal pacakages
brewp asciinema

# Specify taps
brewp homebrew/cask-fonts --tap

# Specify cask pacakages
brewp alacritty --cask

# Install only specified, uninstalled packages
brewp install
```

A full example is placed [here](https://github.com/ken-matsui/dotfiles/blob/main/.config/zsh/software_config/brewp.zsh).
