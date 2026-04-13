# Supported Shells

Funky's goal is to integrate with your shell's native function machinery — not to replace it. Each shell has its own mechanism for defining, loading, and invoking functions, and Funky generates output that speaks each shell's dialect natively.

## Zsh <Badge type="tip" text="fully supported" /> {#zsh}

Zsh is Funky's primary target and the only shell with full support today.

### How It Works

Funky uses zsh's built-in **`fpath` + `autoload`** mechanism. When you run `funky init`, it adds two lines to your `.zshrc`:

```sh
fpath=(~/.funky $fpath)
autoload -Uz $fpath[1]/*(.:t)
```

This tells zsh: "look in `~/.funky/` for function files, and lazy-load them by name." Functions aren't read until you actually call them, so startup stays fast regardless of how many you have.

### Generated Functions

Each function is a standalone `.zsh` file that defines and immediately invokes itself:

```sh
my-func () {
  kubectl apply -f manifests/ --namespace production
}

my-func "$@"
```

This is standard zsh autoload format — nothing Funky-specific. If you deleted Funky entirely, these files would still work as long as your `fpath` points to them.

## Fish <Badge type="warning" text="planned" /> {#fish}

Fish support is in early development. The template infrastructure exists (`template/functions/fish`) but generation isn't wired up yet.

Fish has a natural fit for Funky since it uses a similar model to zsh — functions live as individual files in `~/.config/fish/functions/`. The main work is generating fish-dialect function syntax and writing the init snippet.

### Expected Behavior

```sh
funky init fish --rc-file ~/.config/fish/config.fish
funky new deploy -- kubectl apply -f manifests/
```

Fish functions would be generated as:

```fish
function deploy
    kubectl apply -f manifests/ --namespace production
end
```

## Bash <Badge type="info" text="not yet started" /> {#bash}

Bash support is not yet implemented. Bash's function model is simpler than zsh — functions are typically sourced from a single file (like `.bashrc`) rather than autoloaded from a directory. Funky would need to either:

- Source all function files from a directory in `.bashrc`
- Append function definitions directly to `.bashrc`

The first approach is more aligned with Funky's file-per-function model.

## Adding a New Shell

Funky's architecture makes adding shell support a contained task. See the [Architecture](/development/architecture) page for details, but the short version:

1. Create a template in `template/functions/` with the shell's function syntax
2. Implement the `Function` trait for the new shell (like `Zsh` in `src/functions/zsh.rs`)
3. Add init logic to generate the appropriate rc-file snippet
4. Add the shell name to the CLI argument parser

Each shell is isolated — adding fish doesn't touch any zsh code.
