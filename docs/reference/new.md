# funky new

Create a new shell function from a command.

## Usage

```sh
funky new <NAME> [OPTIONS] [-- <COMMAND>...]
```

## Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `NAME` | Yes | Function name |
| `COMMAND...` | Conditional | Command to save (after `--`) |

## Options

| Option | Default | Description |
|--------|---------|-------------|
| `--from <SOURCE>` | `vargs` | Where to read the command |
| `--history-file <PATH>` | `$HISTFILE` or `~/.zsh_history` | History file for `--from history` |
| `--overwrite` | — | Replace existing function |

## Sources

| Source | Description |
|--------|-------------|
| `vargs` | From arguments after `--` (default) |
| `history` | Last entry in shell history |
| `stdin` | From standard input (pipe) |
| `clipboard` | From system clipboard (planned) |

## Examples

```sh
# From arguments
funky new deploy -- kubectl apply -f manifests/ --namespace prod

# From history
funky new rebuild --from history

# From stdin
echo "docker compose up -d" | funky new up --from stdin

# Overwrite existing
funky new deploy --overwrite -- new-deploy-tool push --env production
```

## Behavior

- If no `--` arguments are provided and stdin is a pipe, Funky reads from stdin automatically
- Duplicate names are rejected unless `--overwrite` is set
