# CI Workflow (`ci.yml`)

> Replaces: `.github/workflows/check.yml`

## Purpose

Primary pull request and push gate. Validates code quality, correctness, and cross-platform compatibility. Uses mise/hk for lint parity with local development, and produces SARIF artifacts for GitHub Code Scanning.

## Triggers

```yaml
on:
  push:
    branches: [main]
  pull_request:
  merge_group:        # merge queue support
  workflow_dispatch:
```

## Concurrency

Cancel in-progress runs when a new push arrives for the same PR:

```yaml
concurrency:
  group: ci-${{ github.head_ref && github.ref || github.run_id }}
  cancel-in-progress: true
```

## Jobs

### 1. `lint` — Format + Lint via mise/hk

**Runs on:** `ubuntu-latest`

**Purpose:** Single source of truth for linting. Runs `hk check` which executes the same clippy + prettier + pkl checks defined in `hk.pkl`.

**Steps:**
1. `actions/checkout@<sha>` with `persist-credentials: false`
2. `Swatinem/rust-cache@<sha>` — cache cargo artifacts
3. `jdx/mise-action@<sha>` — installs rust, hk, pkl from `mise.toml`
4. `hk check` — runs all linters defined in `hk.pkl`

**Why mise/hk instead of raw cargo commands:**
- Identical linting in CI and local dev (hk.pkl is the single config)
- Pre-commit hooks and CI use the same tool
- Adding/changing linters requires only editing `hk.pkl`

**Notes:**
- `MISE_DISABLE_TOOLS` is NOT used here because mise manages hk and pkl, which don't have dedicated GHA actions
- Rust toolchain is also managed by mise (`rust = "stable"` in mise.toml)

### 2. `clippy-sarif` — Clippy SARIF Upload

**Runs on:** `ubuntu-latest`

**Purpose:** Separate from `lint` because hk doesn't produce SARIF. This job generates SARIF from clippy for GitHub Code Scanning integration.

**Permissions:**
```yaml
permissions:
  contents: read
  security-events: write
```

**Steps:**
1. `actions/checkout@<sha>`
2. `dtolnay/rust-toolchain@<sha>` with `components: clippy`
3. `Swatinem/rust-cache@<sha>`
4. Install SARIF tools via pre-built binaries (~3s vs ~3min compiling from source):
   ```yaml
   - uses: taiki-e/install-action@<sha>
     with:
       tool: clippy-sarif,sarif-fmt
   ```
5. Run clippy with SARIF output:
   ```bash
   cargo clippy --all-features --all-targets --message-format=json \
     | clippy-sarif \
     | tee clippy-results.sarif \
     | sarif-fmt
   ```
6. Upload SARIF:
   ```yaml
   - uses: github/codeql-action/upload-sarif@<sha>
     with:
       sarif_file: clippy-results.sarif
       category: clippy
   ```

**Design decision — why a separate job from `lint`:**
- `hk check` validates lint passes/fails (binary gate)
- `clippy-sarif` produces structured findings for GitHub's Security tab
- Different concerns: gate vs. reporting
- Keeps SARIF upload isolated with minimal permissions

### 3. `test` — Build + Test Matrix

**Runs on:** Matrix (see below)

**Purpose:** Verify the project compiles and tests pass across platforms and toolchains. Stable across all OSes is the gate; beta/nightly are informational only.

**Strategy:**
```yaml
strategy:
  fail-fast: false
  matrix:
    include:
      - build: stable-linux
        os: ubuntu-latest
        rust: stable
        required: true
      - build: stable-macos
        os: macos-latest
        rust: stable
        required: true
      - build: stable-windows
        os: windows-latest
        rust: stable
        required: true
      - build: beta
        os: ubuntu-latest
        rust: beta
        required: false
      - build: nightly
        os: ubuntu-latest
        rust: nightly
        required: false
continue-on-error: ${{ !matrix.required }}
```

**Steps:**
1. `actions/checkout@<sha>`
2. `dtolnay/rust-toolchain@<sha>` with `toolchain: ${{ matrix.rust }}`
3. `Swatinem/rust-cache@<sha>` with `shared-key: ${{ matrix.build }}`, `save-if: ${{ github.ref == 'refs/heads/main' }}`
4. `cargo test --workspace --no-fail-fast`

**Design decisions:**
- `fail-fast: false` — a failure on Windows shouldn't cancel the Linux run
- Named builds via `include` — avoids cartesian explosion, each entry is self-documenting
- Beta/nightly only on ubuntu — saves runner minutes, catches upcoming breakage
- Beta/nightly use `continue-on-error` — they surface upcoming issues but never block a merge. The `required` flag drives this.
- Cache saved only on main — PRs restore from main's cache but don't pollute it
- **No MSRV job** — funky is a distributed binary, not a library. No downstream consumers build it with their own toolchain. The only toolchain that matters is stable, which is what we ship with.

### 4. `gate` — Final Status Gate

**Runs on:** `ubuntu-latest`

**Purpose:** Single required status check for branch protection. Aggregates all job results.

**Action:** [`re-actors/alls-green@<sha>`](https://github.com/re-actors/alls-green) — the standard action for this pattern, used by hk, tokio, pydantic, and hundreds of other projects. Handles `continue-on-error` jobs correctly via the `allowed-failures` input.

```yaml
gate:
  if: always()
  needs: [lint, clippy-sarif, test]
  runs-on: ubuntu-latest
  steps:
    - uses: re-actors/alls-green@<sha>
      with:
        jobs: ${{ toJSON(needs) }}
        allowed-failures: test    # beta/nightly use continue-on-error
```

**Why `re-actors/alls-green` over a custom shell script:**
- Correctly distinguishes `success`, `skipped`, `failure`, and `cancelled`
- Handles `continue-on-error` jobs via `allowed-failures` — a matrix entry that used `continue-on-error` reports as `success` even when it failed, which breaks naive shell checks. `alls-green` handles this edge case.
- Battle-tested across thousands of repos
- Single line vs. 10+ lines of brittle bash

**Branch protection config:** Set single required check: `gate`

## Configuration Prerequisites

| File | Change |
|------|--------|
| `hk.pkl` | Ensure `hk check` covers all desired linters (already configured) |
| `mise.toml` | Already configured with hk + rust |
| Branch protection | Set required status check to `gate` |

## Actions Used

All actions pinned to commit SHAs with version comment. Renovate (already configured) sends PRs to update.

| Action | Purpose | Used By |
|--------|---------|---------|
| `actions/checkout` | Repo checkout | All jobs |
| `jdx/mise-action` | Install mise-managed tools (hk, pkl, rust) | `lint` |
| `dtolnay/rust-toolchain` | Install Rust toolchain + components | `clippy-sarif`, `test` |
| `Swatinem/rust-cache` | Cargo build caching | All jobs |
| `taiki-e/install-action` | Install Rust tools from pre-built binaries | `clippy-sarif` (clippy-sarif, sarif-fmt) |
| `github/codeql-action/upload-sarif` | Upload SARIF to GitHub Code Scanning | `clippy-sarif` |
| `re-actors/alls-green` | Aggregate job results for branch protection | `gate` |

```yaml
# Example pinning format:
- uses: actions/checkout@de0fac2e4500dabe0009e67214ff5f5447ce83dd # v6
- uses: Swatinem/rust-cache@e18b497796c12c097a38f9edb9d0641fb99eee32 # v2
- uses: jdx/mise-action@1648a7812b9aeae629881980618f079932869151 # v4
- uses: dtolnay/rust-toolchain@<current-sha> # stable
- uses: taiki-e/install-action@<current-sha> # v2
- uses: re-actors/alls-green@<current-sha> # v1
- uses: github/codeql-action/upload-sarif@<current-sha> # v3
```

## SARIF Outputs

| Source | Category | Upload Action |
|--------|----------|---------------|
| `clippy-sarif` | `clippy` | `github/codeql-action/upload-sarif` |

## Differences from Current `check.yml`

| Aspect | Current | Proposed |
|--------|---------|----------|
| Linting | Raw `cargo fmt` + `cargo clippy` | `hk check` (mise/hk) + separate SARIF job |
| Caching | None | `Swatinem/rust-cache` everywhere |
| Concurrency | None | Cancel-in-progress |
| Matrix strategy | `fail-fast: true` | `fail-fast: false` with named builds |
| Beta/nightly | Block the build | Informational only (`continue-on-error`) |
| MSRV | Not tested | Removed — distributed binary, not a library |
| Action pinning | Tags (`@v6`) | SHA-pinned |
| Gate job | None | `gate` aggregation job |
| `cargo verify-project` | Present | Removed (low value, hk check covers more) |
