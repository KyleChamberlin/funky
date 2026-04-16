# Release Workflow (`release.yml`)

> Replaces: `.github/workflows/create_release.yml`

## Purpose

Automated release pipeline powered by [cargo-dist](https://github.com/axodotdev/cargo-dist) (axo.dev). Triggered via `workflow_dispatch` from the Prep Release workflow. Builds cross-platform binaries, generates checksums and installers, creates signed attestations, and publishes to GitHub Releases.

## Versioning Scheme

**CalVer: `vYYYY.MM.MICRO`**

Examples: `v2026.4.0`, `v2026.4.1`, `v2026.12.0`

- `YYYY` — four-digit year
- `MM` — month (no leading zero)
- `MICRO` — patch/increment within the month, starting at 0

This is a distributed binary, not a library — there are no semver compatibility contracts to uphold. CalVer communicates freshness: consumers immediately know how recent a release is.

### Dynamic Version Resolution

`Cargo.toml` carries a dev placeholder (`version = "0.0.0-dev"`) between releases. The Prep Release workflow (`prep-release.yml`) bumps this to the real CalVer version, commits, tags, and pushes before dispatching the release. cargo-dist then builds against the committed version — no build-time patching needed.

This means **you never manually bump the version in Cargo.toml**. The prep workflow handles it.

## Release Process

```bash
# Option 1: GitHub Actions UI
# Go to Actions → Prep Release → Run workflow

# Option 2: CLI
gh workflow run prep-release.yml
```

The Prep Release workflow:
1. Computes the next CalVer tag (e.g., `v2026.4.0`)
2. Updates `Cargo.toml` and `Cargo.lock` with the version
3. Commits, tags, and pushes to `main`
4. Dispatches `release.yml` with the tag

cargo-dist then runs its full pipeline: plan → build → host → publish → announce.

To update cargo-dist config:

```bash
# Edit [workspace.metadata.dist] in Cargo.toml, then:
dist generate
# Commit the changes — no manual patching needed
# (allow-dirty = ["ci"] tolerates Renovate digest pinning)
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
dispatch-releases = true
allow-dirty = ["ci"]

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

The generated `release.yml` has manual modifications for `workflow_dispatch` trigger support (matching cargo-dist's `dispatch-releases` template). The `allow-dirty = ["ci"]` config in `Cargo.toml` prevents cargo-dist from rejecting these modifications or Renovate digest pinning during PR checks.

When regenerating the workflow:

1. Edit `[workspace.metadata.dist]` in `Cargo.toml`
2. Run `dist generate` to regenerate the workflow
3. Re-apply the `workflow_dispatch` trigger changes (see the git diff of the initial setup for reference)
4. Commit both changes together

To update cargo-dist itself:

```bash
mise upgrade cargo:cargo-dist
dist generate
```

**Note:** The previous `sign-artifacts` job (cosign keyless signing) was not part of cargo-dist's generated output and was removed during regeneration. Re-add it manually if needed.

## SARIF Outputs

None — this workflow produces binaries and attestations, not scan results.

## Configuration Prerequisites

| File | Action |
|------|--------|
| `Cargo.toml` | Add `[workspace.metadata.dist]` + `[profile.dist]` sections (via `cargo dist init`) |
| `Cargo.toml` | Keep `version = "0.0.0-dev"` — bumped by Prep Release workflow before each release |
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
| Version scheme | Unspecified | CalVer `vYYYY.MM.MICRO`, committed to Cargo.toml before release |
| Versioning source | Hardcoded in Cargo.toml | Prep Release workflow bumps Cargo.toml, commits, then dispatches cargo-dist |
| Maintenance | Hand-maintained YAML | `dist generate` (allow-dirty tolerates Renovate pinning) |

## Future Enhancements

- **cargo-auditable:** Add `cargo auditable build` as a custom build step to embed dependency info in binaries (enables post-build vulnerability scanning by Trivy/Grype)
- **Additional targets:** `aarch64-pc-windows-msvc` (Windows ARM) when runners become available
- **Cosign signing:** Re-add the `sign-artifacts` job for keyless cosign signing of release artifacts
