# Commands

| Command | Description |
|---------|-------------|
| [`funky init`](/reference/init) | Configure your shell to load functions |
| [`funky new`](/reference/new) | Create a new function |
| [`funky list`](/reference/list) | List all functions |
| [`funky edit`](/reference/edit) | Edit an existing function |

## Global Options

### `--funky-dir <PATH>`

Directory where Funky stores function files.

- **Default:** `~/.funky/`
- **Env:** `FUNKY_DIR`

```sh
funky --funky-dir ~/my-functions list
```

## Help

```sh
funky --help
funky new --help
```
