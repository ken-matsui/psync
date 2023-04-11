# brewp

A useful tool for managing homebrew packages.

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

`brewp` will automatically install homebrew packages when the shell startup by writing like the following script in your `.zshrc`:

```zsh
# Specify normal pacakages
brewp asciinema

# Specify taps
brewp homebrew/cask-fonts --tap

# Specify cask pacakages
brewp alacritty --cask

# Install only specified, not-installed packages
brewp sync
```

For better startup time for your shell, keep just specifying packages in your `.zshrc` or etc.
When you check whether you have un-synced packages, just run `brew sync` after your shell started.

A full example is placed [here](https://github.com/ken-matsui/dotfiles/blob/main/.config/zsh/software_config/brewp.zsh).
