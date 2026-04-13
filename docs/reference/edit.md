# funky edit

Open an existing function in your editor.

## Usage

```sh
funky edit <NAME> [OPTIONS]
```

## Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `NAME` | Yes | Function to edit |

## Options

| Option | Default | Description |
|--------|---------|-------------|
| `--editor <EDITOR>` | — | Override editor for this invocation |

## Editor Resolution

1. `--editor` flag (highest priority)
2. `$VISUAL`
3. `$EDITOR`
4. `vim`
5. `nano`

## Examples

```sh
funky edit deploy
funky edit deploy --editor "code --wait"
funky edit deploy --editor nano
```

Changes take effect in new shell sessions immediately.
