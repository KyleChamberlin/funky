# vfox Plugin for funky

## Overview

This is a vfox plugin to install and manage funky versions.

## Important

This directory must be extracted to its own Git repository named `vfox-funky`.

## Usage

```sh
vfox add funky
vfox install funky@latest
vfox use funky@latest
```

## Supported platforms

- macOS (x64, aarch64)
- Linux (x64, aarch64, x86)
- Windows (x64, x86)

## Plugin Development

This plugin is based on vfox-plugin-template. Use `vfox --debug install funky@0.0.1` for testing.

## Registry Submission

1. Fork https://github.com/version-fox/vfox-plugins
2. Copy `sources/funky.json` to `sources/funky.json` in the fork
3. Ensure the vfox-funky repo has GitHub Releases with auto-generated `manifest.json`
4. Submit PR

## manifest.json note

If using vfox-plugin-template CI, releases auto-generate `manifest.json`.
