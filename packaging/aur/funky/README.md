# AUR Package: funky (source build)

## Overview

Source build package that compiles funky from source using cargo. This package
downloads the source tarball from GitHub, builds the binary locally, and installs
it to `/usr/bin/funky`.

## First-time AUR Setup

1. Ensure you have an AUR account and SSH key configured at https://aur.archlinux.org
2. Clone the AUR repository:
   ```bash
   git clone ssh://aur@aur.archlinux.org/funky.git
   ```
3. Copy the `PKGBUILD` from this directory into the cloned repo.

## Build Requirements

- Rust stable toolchain
- cargo (pulled in via `makedepends`)
- `gcc-libs` (runtime dependency for dynamically linked Rust binaries)

To test the build locally:

```bash
makepkg -si
```

## Publishing

1. Update `pkgver` in the PKGBUILD when releasing a new version.
2. Regenerate checksums:
   ```bash
   updpkgsums
   ```
3. Generate `.SRCINFO`:
   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```
4. Commit both `PKGBUILD` and `.SRCINFO`, then push to the AUR remote.

## Build Conventions

This PKGBUILD follows the [Arch Wiki Rust packaging guidelines](https://wiki.archlinux.org/title/Rust_package_guidelines):

- **`RUSTUP_TOOLCHAIN=stable`** is exported in `prepare()`, `build()`, and `check()`.
  This prevents the user's default rustup toolchain (e.g., nightly) from interfering
  with the build.

- **`CARGO_TARGET_DIR=target`** is set in `build()`. This overrides any user-level
  cargo configuration that might redirect build output to a non-standard directory,
  ensuring `package()` can find the binary at `target/release/funky`.

- **`--frozen`** is used in `build()` and `check()` instead of `--locked`. The
  `--frozen` flag is equivalent to `--locked --offline`, which provides stronger
  reproducibility guarantees by preventing any network access after `prepare()`.

- **`cargo fetch --locked`** in `prepare()` downloads all dependencies while
  verifying them against the lockfile. This is the only step that accesses the
  network.

- **`check()`** runs the test suite. Arch policy requires running tests when a
  test suite is available.

## Relationship

- `provides=('funky')` — declares this package provides the `funky` virtual package
- `conflicts=('funky-bin')` — cannot be installed alongside the prebuilt binary package

Users can choose between this source build (`funky`) or the prebuilt binary
(`funky-bin`) depending on their preference.
