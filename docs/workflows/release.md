# Release Workflow (`release.yml`)

> Replaces: `.github/workflows/create_release.yml`

## Purpose

Automated release pipeline powered by [cargo-dist](https://github.com/axodotdev/cargo-dist) (axo.dev). Triggered by CalVer tags. Builds cross-platform binaries, generates checksums and installers, creates signed attestations, and publishes to GitHub Releases.

## Versioning Scheme

**CalVer: `vYYYY.MM.MICRO`**

Examples: `v2026.4.0`, `v2026.4.1`, `v2026.12.0`

- `YYYY` — four-digit year
- `MM` — month (no leading zero)
- `MICRO` — patch/increment within the month, starting at 0

This is a distributed binary, not a library — there are no semver compatibility contracts to uphold. CalVer communicates freshness: consumers immediately know how recent a release is.

### Dynamic Version Resolution

`Cargo.toml` carries a dev placeholder (`version = "0.0.0-dev"`). The real version is injected at CI time from the git tag. Each job in the release workflow patches `Cargo.toml` after checkout:

```bash
VERSION="${GITHUB_REF_NAME#v}"
sed -i 's/^version = .*/version = "'"$VERSION"'"/' Cargo.toml
```

This means **you never manually bump the version in Cargo.toml**. The tag is the single source of truth.

## Release Process

```bash
# 1. Tag the release
git tag v2026.4.0

# 2. Push the tag (triggers the release workflow)
git push --tags
```

That's it. The workflow extracts `2026.4.0` from the tag, patches Cargo.toml, and cargo-dist handles the rest.

**Note:** If cargo-dist config changes, regenerate and re-apply the version injection steps:
```bash
dist generate
# Then re-add "Set version from tag" steps to the generated workflow
```

## cargo-dist

cargo-dist generates the entire release workflow. You run `cargo dist init`, configure options, and it produces `.github/workflows/release.yml`. When configuration changes, regenerate with `cargo dist generate`.

### What cargo-dist handles

- Cross-platform builds (Linux, macOS, Windows, ARM)
- Archive creation (tar.gz for Unix, zip for Windows)
- SHA256 checksums for all artifacts
- Shell + PowerShell installer scripts
- Homebrew formula generation and publishing
- cargo-binstall metadata
- GitHub Release creation with changelogs
- Build attestations (SLSA-compatible, via GitHub Artifact Attestations)
- `plan → build → host → publish → announce` pipeline orchestration

### Setup

```bash
cargo install cargo-dist
cargo dist init
# Select:
#   - CI: github
#   - Installers: shell, powershell, homebrew
#   - Targets: (select all desired, see below)
#   - GitHub Attestations: yes
cargo dist generate
# Produces .github/workflows/release.yml
```

### Cargo.toml additions

`cargo dist init` adds these sections to `Cargo.toml`:

```toml
[workspace.metadata.dist]
cargo-dist-version = "0.31.0"
ci = "github"
installers = ["shell", "powershell", "homebrew"]
targets = [
  "aarch64-apple-darwin",
  "aarch64-unknown-linux-gnu",
  "x86_64-apple-darwin",
  "x86_64-unknown-linux-gnu",
  "x86_64-pc-windows-msvc",
  "i686-unknown-linux-gnu",
  "i686-pc-windows-gnu",
]
tap = "kylechamberlin/homebrew-tap"
publish-jobs = ["homebrew"]
github-attestations = true

[profile.dist]
inherits = "release"
```

**Notes on target selection:**
- Matches the current `create_release.yml` target matrix, plus adds `x86_64-pc-windows-msvc` (preferred over `-gnu` for Windows)
- `aarch64-apple-darwin` and `x86_64-apple-darwin` build on native macOS runners (cargo-dist handles this)
- Linux ARM/x86 targets use `cross` internally (cargo-dist manages cross installation)

### Generated workflow structure

cargo-dist generates a multi-job workflow following this pipeline:

```
plan → build-local-artifacts → host (create GH release) → publish-homebrew-formula → announce
```

| Job | Purpose |
|-----|---------|
| `plan` | Compute build matrix, determine which artifacts to produce |
| `build-local-artifacts` | Matrix build across all targets, produce archives + checksums |
| `host` | Create GitHub Release, upload all artifacts, generate attestations |
| `publish-homebrew-formula` | Push updated formula to `kylechamberlin/homebrew-tap` |
| `announce` | Post-release notifications (if configured) |

### Installers generated

| Installer | How consumers use it |
|-----------|---------------------|
| **Shell** (`install.sh`) | `curl --proto '=https' --tlsv1.2 -LsSf https://github.com/kylechamberlin/funky/releases/latest/download/funky-installer.sh \| sh` |
| **PowerShell** (`install.ps1`) | `powershell -ExecutionPolicy ByPass -c "irm https://github.com/kylechamberlin/funky/releases/latest/download/funky-installer.ps1 \| iex"` |
| **Homebrew** | `brew install kylechamberlin/tap/funky` |
| **cargo-binstall** | `cargo binstall funky` (metadata auto-generated) |

### Build attestations

cargo-dist uses `actions/attest-build-provenance` to produce SLSA-compatible attestations for every release artifact. Consumers verify with:

```bash
gh attestation verify funky-v2026.4.0-x86_64-unknown-linux-gnu.tar.gz \
  --repo kylechamberlin/funky
```

### Maintenance

The generated `release.yml` has one manual addition: "Set version from tag" steps that patch `Cargo.toml` at CI time. When regenerating the workflow:

1. Edit `[workspace.metadata.dist]` in `Cargo.toml`
2. Run `dist generate` to regenerate the workflow
3. Re-add the "Set version from tag" steps after each `actions/checkout` in the `plan`, `build-local-artifacts`, `build-global-artifacts`, and `host` jobs
4. Commit both changes together

To update cargo-dist itself:

```bash
mise upgrade cargo:cargo-dist
dist generate
# Re-add version injection steps
```

## SARIF Outputs

None — this workflow produces binaries and attestations, not scan results.

## Configuration Prerequisites

| File | Action |
|------|--------|
| `Cargo.toml` | Add `[workspace.metadata.dist]` + `[profile.dist]` sections (via `cargo dist init`) |
| `Cargo.toml` | Keep `version = "0.0.0-dev"` — real version injected from tag at CI time |
| Homebrew tap repo | **Create** `kylechamberlin/homebrew-tap` on GitHub (empty repo, cargo-dist pushes formula) |
| `.github/workflows/create_release.yml` | **Delete** — replaced by cargo-dist generated workflow |

## Differences from Current `create_release.yml`

| Aspect | Current | Proposed (cargo-dist) |
|--------|---------|----------|
| Build orchestration | Manual matrix + shell scripts | cargo-dist generated pipeline |
| cross version | HEAD from git (unpinned) | Managed by cargo-dist (pinned internally) |
| Checksums | None | SHA256 per-artifact (automatic) |
| Attestation | None | `actions/attest-build-provenance` (SLSA) |
| Archive format | zip for all via 7z | tar.gz for Unix, zip for Windows |
| Windows targets | `-gnu` only | `-msvc` primary |
| Installers | None | Shell, PowerShell, Homebrew |
| Homebrew | None | Auto-published to tap repo |
| cargo-binstall | None | Supported out of the box |
| Release notes | Empty body | Auto-generated from commits |
| Version scheme | Unspecified | CalVer `vYYYY.MM.MICRO`, resolved from tag at CI time |
| Versioning source | Hardcoded in Cargo.toml | Git tag (Cargo.toml patched dynamically) |
| Maintenance | Hand-maintained YAML | `dist generate` + re-add version injection steps |

## Future Enhancements

- **cargo-auditable:** Add `cargo auditable build` as a custom build step to embed dependency info in binaries (enables post-build vulnerability scanning by Trivy/Grype)
- **Additional targets:** `aarch64-pc-windows-msvc` (Windows ARM) when runners become available
- **Release automation:** Consider a mise task to automate the tag + push flow
