# Getting Started

After [installing Funky](/guide/installation), initialize your shell so it knows where to find your functions.

## Initialize

```sh
funky init
```

This appends a small snippet to `~/.zshrc` that adds Funky's function directory to your shell's `fpath` and configures autoloading.

::: info
Only zsh is supported currently. Bash and fish support is planned.
:::

### Custom Locations

```sh
funky --funky-dir ~/my-functions init --rc-file ~/.config/zsh/.zshrc
```

## Verify

Source your rc file (or open a new terminal):

```sh
source ~/.zshrc
```

## Your First Function

```sh
funky new hello -- echo "Hello from Funky!"
```

Try it:

```sh
hello
# Hello from Funky!
```

::: tip Next step
[Learn all the ways to create functions →](/guide/creating-functions)
:::
