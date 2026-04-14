# asdf Plugin for funky

## Overview

asdf plugin to install and manage funky versions.

## Important

This directory must be extracted to its own Git repository named `asdf-funky` to work as an asdf plugin.

## Usage

```bash
asdf plugin add funky https://github.com/KyleChamberlin/asdf-funky.git
asdf install funky latest
asdf global funky latest
```

## Supported platforms

- macOS: x64, aarch64
- Linux: x64, aarch64, x86
- Windows is not supported because asdf does not support it

## Registration

Submit a pull request to https://github.com/asdf-vm/asdf-plugins adding `plugins/funky` with:

```ini
repository = https://github.com/KyleChamberlin/asdf-funky.git
```
