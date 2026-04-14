# Packaging & Distribution

This directory contains packaging artifacts and registry entries for distributing funky through various package managers and version managers.

## Directory Structure

| Directory | Target Registry | Status | Details |
|-----------|----------------|--------|---------|
| [`aqua/`](aqua/README.md) | [aqua-registry](https://github.com/aquaproj/aqua-registry) | Pending submission | aqua registry YAML entry |
| [`mise/`](mise/README.md) | [mise registry](https://github.com/jdx/mise) | Pending submission | mise TOML entry |
| [`asdf-funky/`](asdf-funky/README.md) | [asdf plugin index](https://github.com/asdf-vm/asdf-plugins) | Needs own repo | Complete asdf plugin |
| [`vfox-funky/`](vfox-funky/README.md) | [vfox registry](https://github.com/version-fox/vfox-plugins) | Needs own repo | Complete vfox plugin |
| [`aur/funky-bin/`](aur/funky-bin/README.md) | [AUR](https://aur.archlinux.org) | Pending submission | Pre-compiled binary |
| [`aur/funky/`](aur/funky/README.md) | [AUR](https://aur.archlinux.org) | Pending submission | Source build |

## Cosign Artifact Signing

Release artifacts are signed with cosign keyless signing via [Sigstore](https://www.sigstore.dev/). The signing job is defined in `.github/workflows/release.yml` (the `sign-artifacts` job). Each release asset gets a `.sigstore.json` bundle uploaded alongside it.

To verify a release asset:

```sh
cosign verify-blob \
  --bundle funky-v0.0.1-macos-aarch64.zip.sigstore.json \
  --certificate-identity="https://github.com/KyleChamberlin/funky/.github/workflows/release.yml@refs/tags/v0.0.1" \
  --certificate-oidc-issuer="https://token.actions.githubusercontent.com" \
  funky-v0.0.1-macos-aarch64.zip
```

## Recommended Submission Order

1. **Cosign signing** — automatic on next release (no action needed)
2. **aqua-registry** → **mise registry** — quick PRs, unlocks `mise install funky`
3. **AUR** — requires SSH key setup, see [`aur/README.md`](aur/README.md)
4. **asdf / vfox** — require extracting to separate repos first

## Release Asset Naming Convention

| Platform | Architecture | Asset Name |
|----------|-------------|------------|
| macOS | aarch64 | `funky-v{ver}-macos-aarch64.zip` |
| macOS | x64 | `funky-v{ver}-macos-x64.zip` |
| Linux | aarch64 | `funky-v{ver}-linux-aarch64.zip` |
| Linux | x64 | `funky-v{ver}-linux-x64.zip` |
| Linux | x86 | `funky-v{ver}-linux-x86.zip` |
| Windows | x64 | `funky-v{ver}-windows-x64.zip` |
| Windows | x86 | `funky-v{ver}-windows-x86.zip` |
