# Funky

**Turn command history into reusable shell functions.**

[![CI](https://github.com/KyleChamberlin/funky/actions/workflows/ci.yml/badge.svg)](https://github.com/KyleChamberlin/funky/actions/workflows/ci.yml)
[![License: GPL-3.0-or-later](https://img.shields.io/github/license/KyleChamberlin/funky)](LICENSE.md)
[![GitHub Release](https://img.shields.io/github/v/release/KyleChamberlin/funky)](https://github.com/KyleChamberlin/funky/releases/latest)
[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/KyleChamberlin/funky/badge)](https://securityscorecards.dev/viewer/?uri=github.com/KyleChamberlin/funky)

You've carefully crafted the perfect command &mdash; the right flags, the right switches, the right incantation. Tomorrow you'll scroll through history trying to find it again. Funky saves that command as a native shell function you can call anytime.

```sh
# Save a command
funky new deploy -- kubectl apply -f manifests/ --namespace production

# Use it
deploy
```

No wrappers. No aliases. No runtime. Just a plain shell function.

## Features

- **Zero overhead** &mdash; Functions are native shell functions, as fast as anything your shell can run
- **Instant availability** &mdash; Create a function and use it immediately, no shell restart required
- **Plain files** &mdash; One file per function in `~/.funky/`, easy to version control or sync across machines
- **Multiple sources** &mdash; Capture commands from arguments, shell history, or stdin

## Install

### Homebrew (recommended)

```sh
brew install kylechamberlin/tap/funky
```

### Shell installer

```sh
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/KyleChamberlin/funky/releases/latest/download/funky-installer.sh | sh
```

### Chocolatey (Windows)

```powershell
choco install funky
```

### PowerShell

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/KyleChamberlin/funky/releases/latest/download/funky-installer.ps1 | iex"
```

### cargo-binstall

```sh
cargo binstall funky
```

### From source

```sh
cargo install --git https://github.com/KyleChamberlin/funky.git
```

### aqua

```sh
aqua g -i KyleChamberlin/funky
```

### mise

```sh
mise install funky
mise use funky@latest
```

### asdf

```sh
asdf plugin add funky https://github.com/KyleChamberlin/asdf-funky.git
asdf install funky latest
asdf global funky latest
```

### vfox

```sh
vfox add funky
vfox install funky@latest
```

### AUR (Arch Linux)

```sh
# Pre-compiled binary
yay -S funky-bin

# Or build from source
yay -S funky
```

### Verify signatures

Release artifacts are signed with [cosign](https://docs.sigstore.dev/cosign/signing/overview) keyless signing:

```sh
cosign verify-blob \
  --bundle <artifact>.sigstore.json \
  --certificate-identity="https://github.com/KyleChamberlin/funky/.github/workflows/release.yml@refs/tags/<tag>" \
  --certificate-oidc-issuer="https://token.actions.githubusercontent.com" \
  <artifact>
```

## Quick Start

**1. Initialize your shell**

```sh
funky init
```

This configures zsh to autoload functions from `~/.funky/`. Run it once.

**2. Create a function**

```sh
funky new hello -- echo "Hello from Funky!"
```

**3. Use it**

```sh
hello
# Hello from Funky!
```

## Creating Functions

### From arguments

```sh
funky new deploy -- kubectl apply -f manifests/ --namespace production
```

### From shell history

```sh
docker compose -f docker-compose.prod.yml up -d --build --force-recreate
funky new redeploy --from history
```

### From stdin

```sh
echo "cargo test --workspace --no-fail-fast" | funky new test-all --from stdin
```

### With environment variables

Variables resolve at call time:

```sh
funky new connect -- ssh "$DEPLOY_USER@$DEPLOY_HOST"
```

## Commands

| Command | Description |
|---|---|
| `funky init` | Configure your shell to load functions |
| `funky new <name> [-- <command>]` | Create a new function |
| `funky list` | List all functions |
| `funky edit <name>` | Open a function in your editor |

Use `--funky-dir <path>` to change the function storage directory (default: `~/.funky/`).

## Shell Support

| Shell | Status |
|---|---|
| zsh | Fully supported |
| fish | Planned |
| bash | Planned |

Funky generates native shell functions using each shell's own machinery. If you uninstall Funky, your functions keep working.

## How It Works

Funky uses zsh's built-in `fpath` + `autoload` mechanism. Functions are stored as individual `.zsh` files:

```
~/.funky/
├── deploy.zsh
├── hello.zsh
└── test-all.zsh
```

Each file is a standard zsh autoloadable function &mdash; nothing Funky-specific. Functions load lazily, so startup stays fast regardless of how many you have.

## Documentation

Full docs at **[ktc.sh/funky](https://ktc.sh/funky)**.

## Contributing

```sh
git clone https://github.com/KyleChamberlin/funky.git
cd funky
mise install
cargo build
cargo test --workspace
```

See the [contributing guide](https://ktc.sh/funky/development/contributing) for details.

## License

[GPL-3.0-or-later](LICENSE.md)
