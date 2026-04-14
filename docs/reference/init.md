# funky init

Configure your shell to automatically load Funky-managed functions.

## Usage

```sh
funky init [SHELL] [OPTIONS]
```

## Arguments

| Argument | Default | Description |
|----------|---------|-------------|
| `SHELL` | `zsh` | Target shell |

## Options

| Option | Default | Description |
|--------|---------|-------------|
| `--rc-file <PATH>` | `~/.zshrc` | Shell config file to modify |
| `--completion` | — | *(Not yet implemented)* See [`funky usage`](/reference/usage) for shell completions |

## What It Does

Appends a snippet to your shell rc file that:

1. Adds the Funky directory to `fpath`
2. Enables autoloading via `autoload -Uz`

## Idempotent

Safe to run multiple times — existing config is detected and skipped.

## Examples

```sh
funky init
funky init zsh --rc-file ~/.config/zsh/.zshrc
```

## Shell Support

| Shell | Status |
|-------|--------|
| zsh | ✅ Supported |
| bash | Planned |
| fish | Planned |
