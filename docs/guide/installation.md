# Installation

## Homebrew <Badge type="tip" text="recommended" />

```sh
brew install kylechamberlin/tap/funky
```

## Shell Installer

```sh
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/KyleChamberlin/funky/releases/latest/download/funky-installer.sh | sh
```

## Chocolatey (Windows)

```powershell
choco install funky
```

## PowerShell

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/KyleChamberlin/funky/releases/latest/download/funky-installer.ps1 | iex"
```

## cargo-binstall

```sh
cargo binstall funky
```

## Nix

```sh
nix profile install github:KyleChamberlin/funky
```

Or try it without installing:

```sh
nix run github:KyleChamberlin/funky -- --help
```

## From Source

Requires [Rust](https://rustup.rs/) stable:

```sh
cargo install --git https://github.com/KyleChamberlin/funky.git
```

## Verify

```sh
funky --version
```

::: tip Next step
[Set up your shell →](/guide/getting-started)
:::
