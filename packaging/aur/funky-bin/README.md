# AUR Package: funky-bin

## Overview

Pre-compiled binary package of funky for Arch Linux. This package installs the
pre-built `funky` binary, avoiding the need for a Rust toolchain at install time.

## First-time AUR Setup

1. **Create SSH key:**

   ```bash
   ssh-keygen -t ed25519 -C "aur" -f ~/.ssh/aur
   ```

2. **Add public key to AUR account** at https://aur.archlinux.org (My Account → SSH Public Key)

3. **Configure SSH** — add to `~/.ssh/config`:

   ```
   Host aur.archlinux.org
     IdentityFile ~/.ssh/aur
     User aur
   ```

4. **Clone the AUR repo:**

   ```bash
   git -c init.defaultBranch=master clone ssh://aur@aur.archlinux.org/funky-bin.git
   ```

## Publishing

1. Copy `PKGBUILD` to the cloned AUR directory
2. Update checksums (requires `pacman-contrib`):

   ```bash
   updpkgsums
   ```

3. Generate `.SRCINFO`:

   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```

4. Test the build:

   ```bash
   makepkg -si
   ```

5. Validate:

   ```bash
   namcap PKGBUILD
   ```

6. Commit and push:

   ```bash
   git add PKGBUILD .SRCINFO && git commit -m "funky-bin 0.0.1" && git push
   ```

## Updating

For each new release:

1. Update `pkgver` in PKGBUILD to the new version
2. Reset `pkgrel=1` (if it was bumped)
3. Run `updpkgsums` to fetch new checksums
4. Regenerate `.SRCINFO`: `makepkg --printsrcinfo > .SRCINFO`
5. Test with `makepkg -si`, validate with `namcap PKGBUILD`
6. Commit and push

## Relationship to `funky` (source package)

This package declares `provides=('funky')` and `conflicts=('funky')`, meaning:

- It satisfies any dependency on `funky`
- It cannot be installed alongside the source-build `funky` package
- Users choose one or the other: `funky` (build from source) or `funky-bin` (pre-compiled)
