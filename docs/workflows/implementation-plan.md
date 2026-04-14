# Implementation Plan

## Overview

Four workflows replacing three, with significantly expanded coverage:

| New Workflow | Replaces | Status |
|---|---|---|
| `ci.yml` | `check.yml` | Redesigned |
| `security.yml` | `audit.yml` | Redesigned + expanded |
| `release.yml` | `create_release.yml` | Redesigned |
| `scorecard.yml` | — | New |

**Files to delete:** `check.yml`, `audit.yml`, `create_release.yml`, `.whitesource`

## Implementation Phases

### Phase 0: Prerequisites (do first, no workflow changes)

These are repo-level changes that the new workflows depend on.

| # | Task | File | Details |
|---|------|------|---------|
| 0.1 | Create `deny.toml` | `deny.toml` | cargo-deny configuration for advisories, licenses, bans, sources (see template in `docs/workflows/security.md`) |
| 0.2 | Create `SECURITY.md` | `SECURITY.md` | Vulnerability reporting instructions — improves OpenSSF Scorecard |
| 0.3 | Remove `.whitesource` | `.whitesource` | Replaced by Trivy + cargo-deny |
| 0.4 | Resolve action SHAs | — | Look up current commit SHAs for all actions to be used (for pinning) |

**Estimated effort:** ~1 hour

---

### Phase 1: CI Workflow (`ci.yml`)

**Depends on:** Phase 0 (`deny.toml`)

Implement in this order to allow incremental validation:

| # | Task | Details |
|---|------|---------|
| 1.1 | Create `.github/workflows/ci.yml` scaffold | Triggers, concurrency, permissions |
| 1.2 | Implement `lint` job | mise-action → hk check |
| 1.3 | Implement `clippy-sarif` job | clippy → clippy-sarif → upload-sarif |
| 1.4 | Implement `test` job | Matrix build with named entries, Swatinem/rust-cache. Stable builds are required; beta/nightly use `continue-on-error` (informational only). |
| 1.5 | Implement `gate` job | Aggregation gate for branch protection |
| 1.6 | Delete `.github/workflows/check.yml` | Old workflow removed |
| 1.7 | Update branch protection | Set required check to `gate` |

**Validation:**
- Push to a feature branch, open PR
- Verify `hk check` runs and reports same results as local
- Verify SARIF appears in GitHub Code Scanning (Security tab → Code scanning alerts)
- Verify stable matrix tests pass across all OSes
- Verify beta/nightly failures don't block the gate
- Verify `gate` job reflects the aggregate status

**Estimated effort:** ~2-3 hours

---

### Phase 2: Security Workflow (`security.yml`)

**Depends on:** Phase 0 (`deny.toml`)

| # | Task | Details |
|---|------|---------|
| 2.1 | Create `.github/workflows/security.yml` scaffold | Triggers, path filters, schedule |
| 2.2 | Implement `cargo-deny` job | Install via taiki-e, run with SARIF, upload |
| 2.3 | Implement `trivy` job | trivy-action with SARIF, filesystem scan |
| 2.4 | Implement `semgrep` job | semgrep-action with Rust + security-audit + secrets rulesets |
| 2.5 | Implement `cargo-machete` job | Unused dependency detection |
| 2.6 | Delete `.github/workflows/audit.yml` | Old workflow removed |

**Validation:**
- Run workflow manually via `workflow_dispatch`
- Verify 3 SARIF categories appear in Code Scanning: `cargo-deny`, `trivy`, `semgrep`
- Verify `deny.toml` license allowlist doesn't block current dependencies
- Verify cargo-machete doesn't report false positives (if it does, add ignore annotations)
- Open a PR that touches `Cargo.toml` — verify the workflow triggers

**Estimated effort:** ~2-3 hours

---

### Phase 3: Release Workflow (`release.yml`) — cargo-dist

**Depends on:** Nothing (can be done in parallel with Phase 1-2)

**Versioning:** CalVer `vYYYY.MM.MICRO` (e.g., `v2026.4.0`). Version resolved dynamically from the git tag at CI time — `Cargo.toml` carries `version = "0.0.0-dev"` as a placeholder.

| # | Task | Details |
|---|------|---------|
| 3.1 | Install cargo-dist | `mise use cargo:cargo-dist` |
| 3.2 | Add `[workspace.metadata.dist]` to Cargo.toml | Targets, installers, attestations config |
| 3.3 | Set `Cargo.toml` version to placeholder | `version = "0.0.0-dev"` — real version comes from tag |
| 3.4 | Run `dist generate` | Generates `.github/workflows/release.yml` |
| 3.5 | Add "Set version from tag" steps | Inject `sed` step after each checkout in plan, build-local, build-global, host jobs |
| 3.6 | Create `kylechamberlin/homebrew-funky` repo | Empty repo on GitHub — cargo-dist pushes the formula automatically |
| 3.7 | Delete `.github/workflows/create_release.yml` | Old workflow removed |
| 3.8 | Test with a release tag | Push `v2026.4.0` to validate |

**Validation:**
- Push a tag (e.g., `v2026.4.0`)
- Verify all target builds complete (check each matrix entry in Actions)
- Verify GitHub Release is created with archives, checksums, and installer scripts
- Verify attestation: `gh attestation verify <artifact> --repo kylechamberlin/funky`
- Verify Homebrew formula pushed to tap repo
- Verify shell installer works: `curl ... | sh`
- Verify release notes are auto-populated

**Estimated effort:** ~1-2 hours

---

### Phase 4: Scorecard Workflow (`scorecard.yml`)

**Depends on:** Phase 0 (`SECURITY.md`), ideally after Phase 1 (action pinning improves score)

| # | Task | Details |
|---|------|---------|
| 4.1 | Create `.github/workflows/scorecard.yml` | Full workflow (simple — single job) |
| 4.2 | Run manually via `workflow_dispatch` | Verify SARIF upload + published score |
| 4.3 | Add Scorecard badge to README.md | After first successful published run |

**Validation:**
- Check https://securityscorecards.dev/viewer/?uri=github.com/kylechamberlin/funky
- Verify SARIF appears in Code Scanning under `scorecard` category
- Review score and identify low-hanging improvements

**Estimated effort:** ~30 minutes

---

### Phase 5: Cleanup + Polish

| # | Task | Details |
|---|------|---------|
| 5.1 | Pin all action SHAs | Use Renovate to manage SHA updates going forward |
| 5.2 | Remove `.whitesource` | If not done in Phase 0 |
| 5.3 | Verify Renovate handles SHA pins | Renovate supports `helpers:pinGitHubActionDigests` preset |
| 5.4 | Configure branch protection | Single required check: `gate` from ci.yml |
| 5.5 | Audit SARIF categories in Security tab | Verify all 4 categories present: clippy, cargo-deny, trivy, semgrep, scorecard |
| 5.6 | Document in README | Brief CI section describing what each workflow does |

**Estimated effort:** ~1 hour

---

## Dependency Graph

```
Phase 0 (prerequisites)
  ├── Phase 1 (ci.yml) ──────┐
  ├── Phase 2 (security.yml) ├── Phase 5 (cleanup)
  └── Phase 4 (scorecard.yml)┘
Phase 3 (release.yml) ─ independent, can run in parallel
```

## Total Estimated Effort

| Phase | Effort |
|-------|--------|
| Phase 0: Prerequisites | ~1 hour |
| Phase 1: CI Workflow | ~2-3 hours |
| Phase 2: Security Workflow | ~2-3 hours |
| Phase 3: Release Workflow | ~1-2 hours |
| Phase 4: Scorecard Workflow | ~30 minutes |
| Phase 5: Cleanup | ~1 hour |
| **Total** | **~7-10 hours** |

## Final State

After implementation, the repository will have:

**4 workflows:**
- `ci.yml` — lint (mise/hk), clippy SARIF, test matrix (stable gates, beta/nightly informational), gate
- `security.yml` — cargo-deny, Trivy, Semgrep, cargo-machete
- `release.yml` — cargo-dist generated: cross-platform builds, checksums, attestations, installers, Homebrew
- `scorecard.yml` — OpenSSF Scorecard

**5 SARIF streams in GitHub Code Scanning:**
- `clippy` — lint findings
- `cargo-deny` — dependency advisories + license violations
- `trivy` — vulnerability findings (OSV/NVD sources)
- `semgrep` — SAST findings
- `scorecard` — security posture findings

**Supply chain security:**
- SHA-pinned actions (Renovate-managed)
- Build attestations on releases (SLSA-compatible)
- SHA256 checksums for all release artifacts
- No Dependabot — Renovate handles dependency updates
- cargo-deny enforces license and source policies

**Developer experience:**
- Local and CI linting use the same tool (hk)
- Single `gate` check for branch protection
- Cancel-in-progress for fast PR iteration
- Beta/nightly builds surface upcoming issues without blocking merges
