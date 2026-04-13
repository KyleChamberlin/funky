---
description: >-
  Upgrade all project dependencies, tooling, and Rust edition to latest stable
  releases. Queries canonical sources, applies changes, fixes lint/format issues,
  creates atomic commits, and opens a PR. No human intervention required.
---

Upgrade every versionable artifact in this project to its latest stable release,
verify everything compiles and passes, then ship it as a PR with atomic commits.

Current project state:

!`cat Cargo.toml`

!`cat mise.toml`

!`head -2 hk.pkl`

!`ls .github/workflows/`

## Upgrade Targets

Discover and upgrade ALL of the following categories. For each, query the
**canonical source** to get the true latest version — never guess.

### 1. Rust Crate Dependencies (Cargo.toml)

**Canonical source:** crates.io via `cargo search <crate> --limit 1` and `cargo info <crate>`.

- For each crate in `[dependencies]`, run `cargo search <name> --limit 1` to get the latest stable version (skip pre-releases/alphas).
- `cargo info <name>` gives more detail when `cargo search` returns an alpha as the top result (e.g., tera).
- Update the version string in `Cargo.toml` to the latest stable.
- Run `cargo update` to regenerate `Cargo.lock`.
- Do NOT change feature flags unless a feature was renamed or removed upstream.

### 2. Rust Edition (Cargo.toml)

- If a newer stable edition exists and is supported by the active toolchain (check with `rustc --version`), upgrade the `edition` field.
- After upgrading, run `cargo fmt` (editions change import ordering) and `cargo clippy --all-features --all-targets -- -Dwarnings` to surface new lints.
- Fix all new clippy warnings idiomatically (e.g., collapse nested `if let` chains via let-chains, add missing `Default` impls).

### 3. GitHub Actions Workflow Pinned SHAs

**Canonical source:** GitHub API via `gh api`.

For every `uses:` line in `.github/workflows/*.yml`:
- Extract the action owner/repo and current SHA.
- Look up the latest release tag:
  ```bash
  gh api repos/OWNER/REPO/releases/latest --jq '.tag_name'
  ```
- Resolve the tag to a full 40-char commit SHA (handle annotated tags):
  ```bash
  ref_json=$(gh api repos/OWNER/REPO/git/ref/tags/TAG)
  sha=$(echo "$ref_json" | jq -r '.object.sha')
  type=$(echo "$ref_json" | jq -r '.object.type')
  if [ "$type" = "tag" ]; then
    sha=$(gh api "repos/OWNER/REPO/git/tags/$sha" --jq '.object.sha')
  fi
  ```
- For repos without versioned releases (e.g., `dtolnay/rust-toolchain`), get the latest commit on the default branch:
  ```bash
  gh api repos/OWNER/REPO/commits/master --jq '.sha'
  ```
- Update the SHA in the workflow file. Preserve the existing `# vX.Y.Z` or `# vN` comment style.
- Skip any action whose resolved SHA already matches.

### 4. hk.pkl Version

**Canonical source:** `gh api repos/jdx/hk/releases/latest --jq '.tag_name'`

- Check the `amends` and `import` lines in `hk.pkl` for the current version.
- If a newer version exists, update both package URIs.

### 5. mise.toml Tool Versions

**Canonical source:** `gh api repos/OWNER/REPO/releases/latest` for each tool.

- Check `mise.toml` for pinned versions (not `latest`/`stable`).
- If any tool is pinned to a specific version, look up the latest and update.
- Tools set to `latest` or `stable` need no change.

### 6. cargo-dist Version (Cargo.toml metadata + release workflow)

**Canonical source:** `gh api repos/axodotdev/cargo-dist/releases/latest --jq '.tag_name'`

- Check `[workspace.metadata.dist].cargo-dist-version` in `Cargo.toml`.
- Check the installer URL in the release workflow.
- If a newer version exists, update both locations.
- CAUTION: cargo-dist major upgrades can change the release workflow structure. If the version bumps a major, regenerate with `dist init` and verify.

## Verification (MANDATORY before committing)

Run ALL of these and fix any failures before proceeding to commits:

```bash
cargo fmt                                                # Fix formatting first
cargo clippy --all-features --all-targets -- -Dwarnings  # Zero warnings
cargo test --workspace --no-fail-fast                    # All tests pass
cargo fmt --check                                        # Confirm clean
```

If clippy or tests fail:
1. Fix the issues (edition idiom changes, API changes in upgraded crates).
2. Re-run the full verification suite.
3. Do NOT suppress warnings with `#[allow(...)]` unless genuinely inapplicable.

## Atomic Commits

Stash or stage selectively to create **one commit per logical upgrade group**:

1. **Rust edition upgrade** — `Cargo.toml` edition field + all Rust source changes (clippy fixes, reformatting)
2. **Crate dependency upgrades** — `Cargo.toml` dependency versions + `Cargo.lock`
3. **GitHub Actions SHA updates** — `.github/workflows/*.yml`
4. **hk version upgrade** — `hk.pkl`
5. **cargo-dist upgrade** — `Cargo.toml` metadata + release workflow (only if version changed)
6. **mise.toml upgrades** — `mise.toml` (only if any pinned versions changed)

Skip any group where nothing changed.

Commit message style — imperative mood, sentence case:
```
Upgrade Rust edition from 2021 to 2024

Apply edition 2024 idioms: collapse nested if-let chains,
add Default impl for MockRepository, and reformat imports
per the new edition's style rules.
```

## PR Creation

After all commits pass verification:

1. Create a new branch `upgrade/deps-and-tooling` from the default branch.
2. Push with tracking: `git push -u origin upgrade/deps-and-tooling`.
3. Open a PR with `gh pr create` summarizing each upgrade category and the verification results.

## Workflow Order

1. **Explore** — Read `Cargo.toml`, `mise.toml`, `hk.pkl`, `.github/workflows/*.yml` to discover all targets.
2. **Research** — Query canonical sources for latest versions. Parallelize API calls.
3. **Branch** — Create upgrade branch from the default branch.
4. **Apply** — Make all changes.
5. **Verify** — Run the full check suite. Fix issues iteratively until green.
6. **Commit** — One atomic commit per logical group. Stash/restore to isolate each.
7. **Ship** — Push and open PR.
