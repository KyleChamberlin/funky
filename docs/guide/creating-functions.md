# Creating Functions

Funky gives you multiple ways to capture commands as reusable functions.

## From Arguments (default)

Pass your command after `--`:

```sh
funky new deploy -- kubectl apply -f manifests/ --namespace production
```

Everything after `--` becomes the function body.

### Quoting Special Characters

```sh
funky new greet -- echo "Hello, $USER! Today is $(date +%A)"
```

## From Shell History

Grab the last command you ran:

```sh
# Run your complex command
docker compose -f docker-compose.prod.yml up -d --build --force-recreate

# Then capture it
funky new redeploy --from history
```

## From Standard Input

Pipe a command in:

```sh
echo "cargo test --workspace --no-fail-fast" | funky new test-all --from stdin
```

## Overwriting

Funky refuses to overwrite by default:

```sh
funky new deploy -- new command
# Error: Function 'deploy' already exists. Use --overwrite to replace it.
```

Pass `--overwrite` to replace:

```sh
funky new deploy --overwrite -- new-deploy-tool push --env production
```

## Environment Variables

Reference variables that resolve at call time:

```sh
funky new connect -- ssh "$DEPLOY_USER@$DEPLOY_HOST"
```

Each call to `connect` uses the current values of `$DEPLOY_USER` and `$DEPLOY_HOST`.

## Tips

::: tip Naming
Keep names short — you'll type them often. `d` beats `deploy-production-cluster`. Hyphens work fine as function names.
:::

::: tip Version control
Symlink `~/.funky/` to a git-tracked directory. Your functions become portable and backed up for free.
:::
