# Managing Functions

## Listing

```sh
funky list
```

Displays every function stored in your Funky directory.

## Editing

Open a function in your editor:

```sh
funky edit deploy
```

Editor resolution order:

1. `--editor` flag
2. `$VISUAL`
3. `$EDITOR`
4. `vim`
5. `nano`

Specify an editor directly:

```sh
funky edit deploy --editor "code --wait"
```

## File Layout

Functions live as individual `.zsh` files in `~/.funky/`:

```
~/.funky/
├── deploy.zsh
├── hello.zsh
└── test-all.zsh
```

Each file is a plain-text shell function definition. You can read or edit them directly.

## Removing

Delete the file:

```sh
rm ~/.funky/my-function.zsh
```

The function persists in your current session but won't load in new ones.

## Syncing Across Machines

Functions are plain files — sync them however you like:

- **Git** — track `~/.funky/` in a dotfiles repo
- **Symlink** — point to a synced directory (Dropbox, iCloud, etc.)
- **Copy** — `rsync` the directory to another machine

Functions are immediately available on the target machine after syncing (assuming `funky init` has been run there).
