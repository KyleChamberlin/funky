# Contributing

## Prerequisites

- [Rust](https://rustup.rs/) stable
- [mise](https://mise.jdx.dev/) (manages project tooling)

## Setup

```sh
git clone https://github.com/KyleChamberlin/funky.git
cd funky
mise install
cargo build
```

### With Nix

If you use [Nix](https://nixos.org/), the flake provides a dev shell with all dependencies:

```sh
git clone https://github.com/KyleChamberlin/funky.git
cd funky
nix develop
cargo build
```

## Tests

```sh
cargo test --workspace
```

## Linting

The project uses [hk](https://github.com/jdx/hk) for lint orchestration:

```sh
hk check
```

This runs `cargo fmt`, `cargo clippy`, and other checks configured in `hk.pkl`.

## Making Changes

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `cargo test` and `hk check`
5. Open a pull request

## Releases

[CalVer](https://calver.org/) `vYYYY.MM.MICRO` via [cargo-dist](https://opensource.axo.dev/cargo-dist/):

```sh
git tag v2026.4.0
git push --tags
```

CI handles cross-platform builds, installers, and publishing.

## License

[GPL-3.0-or-later](https://www.gnu.org/licenses/gpl-3.0.html)
