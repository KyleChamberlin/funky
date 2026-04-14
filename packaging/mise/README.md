# mise Registry Entry for funky

## Overview

`funky.toml` is the registry entry to submit to the [mise tool registry](https://github.com/jdx/mise) so users can install funky via `mise use funky`.

## Submission Process

1. Fork https://github.com/jdx/mise
2. Copy `funky.toml` to `registry/funky.toml` in your fork
3. Test locally: `mise test-tool funky`
4. Submit a PR with title: `registry: add funky (github:KyleChamberlin/funky)`

## Phase 1 vs Phase 2

**Phase 1** (current): Uses `github` and `cargo` backends — works immediately without any external dependencies.

**Phase 2** (after aqua-registry PR merges): Adds `aqua:KyleChamberlin/funky` as the primary backend. The `aqua` backend enables cosign/SLSA/GitHub Attestation verification for supply-chain security. To upgrade, uncomment the Phase 2 `backends` block in `funky.toml` and remove the Phase 1 line.

## Backend Priority

mise resolves backends in order — it prefers `aqua` > `github` > `cargo`. New `asdf` and `vfox` entries are rarely accepted by the mise registry maintainers.

> **Note:** The `ubi` backend is deprecated — use `github` instead.
