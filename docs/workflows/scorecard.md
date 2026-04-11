# OpenSSF Scorecard Workflow (`scorecard.yml`)

> New workflow

## Purpose

Runs the [OpenSSF Scorecard](https://securityscorecards.dev/) to evaluate the project's security posture against industry best practices. Produces a score, badge, and SARIF findings for GitHub Code Scanning.

## Why

- Identifies supply chain risks and security hygiene gaps automatically
- Provides an actionable checklist for improving project security
- Produces a public badge demonstrating security commitment
- Used by Node.js, Bootstrap, Express, Material-UI, PowerShell, and hundreds of major OSS projects
- Free, non-Microsoft tool (OpenSSF / Linux Foundation)
- Native SARIF output

## Triggers

```yaml
on:
  schedule:
    - cron: '0 6 * * 1'   # weekly Monday 6am UTC
  push:
    branches: [main]
  workflow_dispatch:
```

**Design:** Weekly schedule plus push to main. Not on PRs — Scorecard evaluates the repository's overall posture (branch protection, CI config, dependency management), which doesn't change per-PR.

## Job

### `scorecard` — OpenSSF Scorecard Analysis

**Runs on:** `ubuntu-latest`

**Permissions:**
```yaml
permissions:
  contents: read
  security-events: write
  id-token: write    # for Scorecard API publishing
```

**Steps:**
```yaml
- uses: actions/checkout@<sha>
  with:
    persist-credentials: false

- name: Run OpenSSF Scorecard
  uses: ossf/scorecard-action@<sha>
  with:
    results_file: scorecard-results.sarif
    results_format: sarif
    publish_results: true    # publish to securityscorecards.dev

- name: Upload Scorecard SARIF
  uses: github/codeql-action/upload-sarif@<sha>
  with:
    sarif_file: scorecard-results.sarif
    category: scorecard
```

**Notes:**
- `persist-credentials: false` on checkout — Scorecard checks for this as part of its "Token Permissions" evaluation
- `publish_results: true` — enables the public scorecard badge and API
- `id-token: write` — required for Scorecard to publish results to the public dashboard

## What Scorecard Checks

Scorecard evaluates ~18 checks. The most actionable ones for funky:

| Check | What It Evaluates | Current Status (Estimated) |
|-------|-------------------|---------------------------|
| **Branch-Protection** | Protected branches, required reviews | Unknown |
| **CI-Tests** | CI runs on PRs | Pass (check.yml exists) |
| **Code-Review** | PRs require review | Unknown |
| **Dangerous-Workflow** | Unsafe patterns in workflows | Likely pass |
| **Dependency-Update-Tool** | Automated dep updates | Pass (Renovate configured) |
| **License** | OSS license present | Pass (GPL-3.0) |
| **Maintained** | Recent activity | Pass (if active) |
| **Pinned-Dependencies** | Actions pinned to SHA | Fail (currently tag-based) |
| **SAST** | Static analysis in CI | Pass (clippy + will add Semgrep) |
| **Security-Policy** | SECURITY.md exists | Likely fail |
| **Signed-Releases** | Signed/attested releases | Fail (no attestations yet) |
| **Token-Permissions** | Minimal workflow permissions | Partial |
| **Vulnerabilities** | Known vulns in deps | Unknown |

## Badge

After first run with `publish_results: true`, add to `README.md`:

```markdown
[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/kylechamberlin/funky/badge)](https://securityscorecards.dev/viewer/?uri=github.com/kylechamberlin/funky)
```

## SARIF Outputs

| Source | Category | Upload Action |
|--------|----------|---------------|
| Scorecard | `scorecard` | `github/codeql-action/upload-sarif` |

## Configuration Prerequisites

| File | Action |
|------|--------|
| `SECURITY.md` | **Create** — security policy/vulnerability reporting instructions (improves Scorecard) |
| None | No additional config needed — Scorecard reads repo metadata |

## Cost

Free — OpenSSF Scorecard is an open-source Linux Foundation project. Public results publishing is free for all public repositories.
