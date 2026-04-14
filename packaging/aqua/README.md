# aqua Registry Entry for funky

## Overview

These files define an [aqua](https://aquaproj.github.io/) registry entry for installing funky via `aqua install`. They are intended for submission to the [aqua-registry](https://github.com/aquaproj/aqua-registry).

## Files

- **registry.yaml** — Package definition describing how aqua downloads and installs funky from GitHub Releases. Includes platform mappings, asset naming patterns, and version overrides.
- **pkg.yaml** — Pin file specifying which version to test during registry CI.

## Submission Process

1. Fork https://github.com/aquaproj/aqua-registry
2. Copy these files to `pkgs/KyleChamberlin/funky/`
3. Run `argd s KyleChamberlin/funky` (MANDATORY — PRs without scaffolding are rejected)
4. Note: `argd s` may generate different code than what's here. Use these files as reference for the correct asset patterns and replacements.
5. Run `argd t` to test the package installs correctly
6. Submit PR

## v0.0.1 Typo Note

The v0.0.1 release assets use `aarach64` (typo) instead of `aarch64` for the ARM64 Linux binary. The registry handles this with a version override:

- `version_constraint: semver("<= 0.0.1")` — uses `arm64: aarach64` to match the misnamed asset
- `version_constraint: "true"` (catch-all for future releases) — uses `arm64: aarch64` (correct spelling)

This ensures both the existing v0.0.1 release and all future releases install correctly.

## Testing

```sh
aqua i -l && aqua g -i KyleChamberlin/funky
```
