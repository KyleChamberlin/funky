# Sources

When you create a function with `funky new`, you need to give it a command. Funky can read that command from several places.

## Command-Line Arguments (default) {#arguments}

Pass the command directly after `--`:

```sh
funky new deploy -- kubectl apply -f manifests/ --namespace production
```

Everything after `--` is joined into the function body. This is the most common path — you already know the command, you just want to save it.

If your command contains characters your shell would interpret, quote them:

```sh
funky new greet -- echo "Hello, $USER! Today is $(date +%A)"
```

### Implicit Stdin Fallback

When using the default `vargs` source and no arguments follow `--`, Funky checks if stdin is a pipe. If it is, it reads from there. This means piping works without `--from stdin`:

```sh
echo "cargo test --workspace" | funky new test-all
```

## Shell History {#history}

```sh
funky new rebuild --from history
```

Funky reads the last line from your history file. The file is determined by:

1. `--history-file` flag (if provided)
2. `$HISTFILE` environment variable
3. `~/.zsh_history` (fallback)

This is ideal when you've just run a command and realize you want to keep it — no need to retype or copy-paste.

::: info History format
Funky currently reads plain-format history files where each line is a single command. Extended history formats with timestamps and metadata (like zsh's `EXTENDED_HISTORY` option) are not yet parsed.
:::

## Standard Input {#stdin}

```sh
echo "docker compose up -d --build" | funky new up --from stdin
```

Explicitly read from stdin. Useful when generating commands from scripts, pulling them from documentation, or chaining with other tools:

```sh
# From a file
cat my-command.txt | funky new run-it --from stdin

# From another program
some-tool --generate-command | funky new generated --from stdin
```

Funky rejects empty input and will error if stdin is a terminal (nothing piped).

## Clipboard {#clipboard}

::: warning Coming Soon
Clipboard support is defined but not yet implemented. Track progress on [GitHub](https://github.com/KyleChamberlin/funky).
:::

The intent is to read the command directly from your system clipboard:

```sh
# Copy a command from a webpage or chat, then:
funky new saved --from clipboard
```

## Choosing a Source

| Source | Best for |
|--------|----------|
| Arguments | You know the command and can type it now |
| History | You just ran it and want to keep it |
| Stdin | Piping from files, scripts, or other tools |
| Clipboard | Grabbing commands from outside the terminal |
