# AUR Packages for funky

## Overview

Two AUR packages are maintained for installing funky on Arch Linux:

- **`funky-bin`** — Pre-compiled binary package (recommended for most users)
- **`funky`** — Source build from the official repository

## Package Comparison

| Package | Method | Dependencies | Build time |
|---------|--------|-------------|------------|
| funky-bin | Pre-compiled binary | gcc-libs | Instant |
| funky | Source build | cargo, gcc-libs | ~2 min |

## Initial AUR Setup

1. Create an AUR account at https://aur.archlinux.org

2. Generate an SSH key:

   ```bash
   ssh-keygen -t ed25519 -C "aur" -f ~/.ssh/aur
   ```

3. Add the public key (`~/.ssh/aur.pub`) to your AUR account under "My Account" → "SSH Public Key"

4. Configure SSH by adding the following to `~/.ssh/config`:

   ```
   Host aur.archlinux.org
     IdentityFile ~/.ssh/aur
     User aur
   ```

5. Clone each package repository:

   ```bash
   git clone ssh://aur@aur.archlinux.org/funky-bin.git
   git clone ssh://aur@aur.archlinux.org/funky.git
   ```

## Automated Updates

A GitHub Actions workflow is provided to automatically update both AUR packages when a new release is published.

1. Copy `.github/workflows/aur-publish.yml` to your funky repo's `.github/workflows/` directory

2. Add the `AUR_SSH_KEY` secret to your GitHub repository settings (paste the private key content from `~/.ssh/aur`)

3. The workflow runs automatically on each GitHub Release (triggered by the `published` event)

4. Can also be triggered manually via `workflow_dispatch` — go to Actions → "Publish AUR Packages" → "Run workflow" and enter the version number (without the `v` prefix)

## Manual Update Process

If you need to update the AUR packages manually:

1. Update `pkgver` in `PKGBUILD`:

   ```bash
   sed -i 's/^pkgver=.*/pkgver=NEW_VERSION/' PKGBUILD
   ```

2. Update checksums:

   ```bash
   updpkgsums
   ```

3. Regenerate `.SRCINFO`:

   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```

4. Test the package builds and installs correctly:

   ```bash
   makepkg -si
   ```

5. Commit and push to AUR:

   ```bash
   git add PKGBUILD .SRCINFO
   git commit -m "Update to vNEW_VERSION"
   git push
   ```

## Package Details

- [`funky-bin/`](funky-bin/) — Binary package PKGBUILD and documentation
- [`funky/`](funky/) — Source package PKGBUILD and documentation
