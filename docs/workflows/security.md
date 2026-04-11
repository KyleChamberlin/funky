# Security Workflow (`security.yml`)

> Replaces: `.github/workflows/audit.yml`

## Purpose

Comprehensive security scanning pipeline covering dependency vulnerabilities, license compliance, SAST, supply chain analysis, and code quality. All findings reported via SARIF to GitHub Code Scanning where supported.

## Triggers

```yaml
on:
  push:
    branches: [main]
    paths:
      - '.github/workflows/security.yml'
      - '**/Cargo.toml'
      - '**/Cargo.lock'
      - 'deny.toml'
      - '**/*.rs'
  pull_request:
    paths:
      - '**/Cargo.toml'
      - '**/Cargo.lock'
      - 'deny.toml'
      - '**/*.rs'
  schedule:
    - cron: '0 6 * * 1'   # weekly Monday 6am UTC
  workflow_dispatch:
```

**Design:** Runs on PRs (for Rust/dependency changes), on push to main, weekly on schedule, and manually. The scheduled run catches newly disclosed vulnerabilities even without code changes.

## Jobs

### 1. `cargo-deny` — Dependency Audit + License + Bans + Sources

**Replaces:** `actions-rust-lang/audit` (cargo-audit)

**Why cargo-deny over cargo-audit:**
- **Superset** — cargo-deny includes everything cargo-audit does (RustSec advisory checking) plus license compliance, crate banning, and source verification
- **Native SARIF** — `--format sarif` for all four check types
- **Single tool** — one config file (`deny.toml`) governs all dependency policies
- **Mature** — by Embark Studios, widely adopted, active maintenance

**Runs on:** `ubuntu-latest`

**Permissions:**
```yaml
permissions:
  contents: read
  security-events: write
```

**Steps:**
1. `actions/checkout@<sha>`
2. Run cargo-deny checks via the official action:
   ```yaml
   - uses: EmbarkStudios/cargo-deny-action@<sha>
     with:
       command: check
       arguments: --all-features
       command-arguments: advisories licenses bans sources
   ```
3. Run cargo-deny again for SARIF output (the action doesn't support SARIF upload directly, so a second pass produces the file):
   ```yaml
   - name: Install cargo-deny
     uses: taiki-e/install-action@<sha>
     with:
       tool: cargo-deny
   - name: Generate SARIF
     run: cargo deny check --format sarif --all-features advisories licenses bans sources > deny-results.sarif 2>&1 || true
   - name: Upload SARIF
     uses: github/codeql-action/upload-sarif@<sha>
     if: always()
     with:
       sarif_file: deny-results.sarif
       category: cargo-deny
   ```

**Why two passes:** `EmbarkStudios/cargo-deny-action` provides nice inline annotations and log formatting but doesn't output SARIF. The SARIF pass uses `taiki-e/install-action` for a fast pre-built binary install and writes the file for Code Scanning upload. The second pass adds ~5s — cargo-deny is fast and the advisory DB is already cached.

**Configuration: `deny.toml`**

A new file at the repo root. Format is for cargo-deny v0.19+:

```toml
[graph]
all-features = true

[advisories]
unmaintained = "workspace"
unsound = "workspace"
yanked = "warn"
ignore = []

[licenses]
allow = [
  "0BSD",
  "Apache-2.0",
  "Apache-2.0 WITH LLVM-exception",
  "BSD-2-Clause",
  "BSD-3-Clause",
  "GPL-3.0-or-later",
  "ISC",
  "LGPL-2.1-or-later",
  "MIT",
  "MPL-2.0",
  "Unicode-3.0",
  "Unlicense",
  "Zlib",
]

[bans]
multiple-versions = "warn"
wildcards = "deny"

[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
allow-git = []
```

### 2. `trivy` — Filesystem Vulnerability Scan

**Purpose:** Secondary vulnerability scanner using a different advisory database (OSV + NVD + vendor DBs). Catches findings that RustSec alone might miss. Scans `Cargo.lock` for known vulnerabilities.

**Why Trivy:**
- Free and open source (Apache-2.0, Aqua Security)
- Native SARIF output
- Scans Cargo.lock + detects secrets + IaC misconfigs
- 25k+ stars, production-grade
- Non-Microsoft/non-GitHub — diversifies advisory sources beyond RustSec

**Runs on:** `ubuntu-latest`

**Permissions:**
```yaml
permissions:
  contents: read
  security-events: write
```

**Steps:**
```yaml
- uses: actions/checkout@<sha>
- name: Run Trivy filesystem scan
  uses: aquasecurity/trivy-action@<sha>
  with:
    scan-type: fs
    scan-ref: .
    format: sarif
    output: trivy-results.sarif
    severity: CRITICAL,HIGH,MEDIUM
- name: Upload Trivy SARIF
  uses: github/codeql-action/upload-sarif@<sha>
  if: always()
  with:
    sarif_file: trivy-results.sarif
    category: trivy
```

**Notes:**
- `severity: CRITICAL,HIGH,MEDIUM` — filters out LOW/UNKNOWN noise
- `if: always()` on upload — ensures SARIF is uploaded even if Trivy finds issues (non-zero exit)
- No configuration file needed — works out of the box with `Cargo.lock`

### 3. `semgrep` — Static Application Security Testing (SAST)

**Purpose:** Pattern-based static analysis of Rust source code. Finds security anti-patterns, bug-prone code, and potential vulnerabilities in the application logic itself (not just dependencies).

**Why Semgrep:**
- Free Community Edition (LGPL-2.1)
- Full Rust support including reachability analysis (as of April 2026)
- Native SARIF output
- 700+ community Rust rules
- Non-Microsoft/non-GitHub
- Industry-leading SAST tool alongside CodeQL

**Runs on:** `ubuntu-latest`

**Permissions:**
```yaml
permissions:
  contents: read
  security-events: write
```

**Steps:**
```yaml
- uses: actions/checkout@<sha>
- name: Run Semgrep
  uses: semgrep/semgrep-action@<sha>
  with:
    config: >-
      p/rust
      p/security-audit
      p/secrets
  env:
    SEMGREP_SARIF_OUTPUT: semgrep-results.sarif
- name: Upload Semgrep SARIF
  uses: github/codeql-action/upload-sarif@<sha>
  if: always()
  with:
    sarif_file: semgrep-results.sarif
    category: semgrep
```

**Rulesets explained:**
- `p/rust` — Rust-specific security and correctness rules
- `p/security-audit` — language-agnostic security patterns
- `p/secrets` — hardcoded secrets/credentials detection

**Notes:**
- No Semgrep account required for Community Edition rules
- For additional rules, a free Semgrep AppSec Platform account can be created (optional)
- Runs in ~30-60 seconds for a project this size

### 4. `cargo-machete` — Unused Dependencies

**Purpose:** Detects unused dependencies. Keeps the dependency tree lean, reducing attack surface and compile times.

**Why cargo-machete over cargo-udeps:**
- Works on stable Rust (cargo-udeps requires nightly)
- Fast heuristic-based analysis
- Has a dedicated GitHub Action
- Good enough for CI — false positives are rare and can be annotated

**Runs on:** `ubuntu-latest`

**Steps:**
```yaml
- uses: actions/checkout@<sha>
- name: Check for unused dependencies
  uses: bnjbvr/cargo-machete@<sha>
```

**Notes:**
- No SARIF output (exit code based: 0 = clean, 1 = unused found)
- If false positives occur, add `# cargo-machete: ignore` comments or configure `[package.metadata.cargo-machete]` in `Cargo.toml`

## SARIF Outputs Summary

| Job | Tool | SARIF Category | Advisory Source |
|-----|------|----------------|-----------------|
| `cargo-deny` | cargo-deny | `cargo-deny` | RustSec |
| `trivy` | Trivy | `trivy` | OSV, NVD, vendor DBs |
| `semgrep` | Semgrep CE | `semgrep` | Semgrep community rules |

Three independent SARIF streams feeding GitHub's Security tab, from three non-Microsoft tools.

## Actions Used

| Action | Purpose | Used By |
|--------|---------|---------|
| `actions/checkout` | Repo checkout | All jobs |
| `EmbarkStudios/cargo-deny-action` | Run cargo-deny checks with inline annotations | `cargo-deny` |
| `taiki-e/install-action` | Install cargo-deny binary for SARIF pass | `cargo-deny` |
| `aquasecurity/trivy-action` | Filesystem vulnerability scan | `trivy` |
| `semgrep/semgrep-action` | SAST analysis | `semgrep` |
| `bnjbvr/cargo-machete` | Unused dependency detection | `cargo-machete` |
| `github/codeql-action/upload-sarif` | Upload SARIF to Code Scanning | `cargo-deny`, `trivy`, `semgrep` |

## Configuration Prerequisites

| File | Action |
|------|--------|
| `deny.toml` | **Create** — cargo-deny configuration (see template above) |
| `.whitesource` | **Remove** — replaced by Trivy + cargo-deny |
| `.github/workflows/audit.yml` | **Delete** — replaced by this workflow |

## Differences from Current `audit.yml`

| Aspect | Current | Proposed |
|--------|---------|----------|
| Tool | cargo-audit only | cargo-deny + Trivy + Semgrep + cargo-machete |
| Scope | Dependency vulnerabilities only | Vulns + licenses + SAST + unused deps + secrets |
| SARIF | None | 3 SARIF streams to Code Scanning |
| Advisory sources | RustSec only | RustSec + OSV + NVD + Semgrep rules |
| Schedule | Daily at midnight | Weekly Monday 6am UTC (daily is excessive for this project size) |
| PR integration | Not on PRs | Runs on PRs touching Rust/dependency files |

## Cost

All tools are free for open source / public repositories:
- cargo-deny: MIT/Apache-2.0
- Trivy: Apache-2.0
- Semgrep CE: LGPL-2.1
- cargo-machete: MIT
- GitHub Code Scanning SARIF upload: free for public repos
