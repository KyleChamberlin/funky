# funky usage

Output the CLI spec in [usage](https://usage.jdx.dev/) format. This powers shell completions, documentation generation, and man page generation.

## Usage

```sh
funky usage
```

## Output

Prints a [KDL](https://kdl.dev/)-formatted spec describing Funky's commands, flags, and arguments. The spec is auto-generated from the CLI definition, so it stays in sync with the actual binary.

## Shell Completions

Requires the [`usage`](https://usage.jdx.dev/) CLI. Install via mise:

```sh
mise use -g usage
```

### Zsh

```sh
funky usage | usage generate completion zsh funky > _funky
```

### Bash

```sh
funky usage | usage generate completion bash funky > funky.bash
```

### Fish

```sh
funky usage | usage generate completion fish funky > funky.fish
```

## Documentation Generation

### Markdown

```sh
funky usage | usage generate markdown --file - --out-file docs.md
```

### Man Page

```sh
funky usage | usage generate manpage --file - --out-file funky.1
```

## Dynamic Completions

The `usage` completion scripts call `funky usage` at completion time, so completions always reflect the installed version of Funky. No need to regenerate after upgrades.

```sh
# Install zsh completions that stay up to date automatically
usage generate completion --usage-cmd "funky usage" zsh funky > _funky
```
