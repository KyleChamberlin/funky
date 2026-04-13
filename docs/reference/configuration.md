# Configuration

Funky works with minimal configuration. Most behavior is controlled through flags and environment variables.

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `FUNKY_DIR` | `~/.funky/` | Function file storage |
| `HISTFILE` | `~/.zsh_history` | History file for `--from history` |
| `VISUAL` | — | Preferred editor (GUI-capable) |
| `EDITOR` | — | Fallback editor |

## Function Storage

```
~/.funky/
├── deploy.zsh
├── hello.zsh
└── test-all.zsh
```

Each file contains a complete zsh function definition that can be autoloaded.

## Custom Directory

Set globally:

```sh
export FUNKY_DIR="$HOME/.config/funky"
```

Or per-command:

```sh
funky --funky-dir ~/work-functions new build -- make release
```

## History File Format

When using `--from history`, Funky reads the last line of the history file. Only plain-format history files are supported (one command per line). Extended history formats with timestamps are not yet parsed.
