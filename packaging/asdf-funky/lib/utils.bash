#!/usr/bin/env bash

set -euo pipefail

GH_REPO="https://github.com/KyleChamberlin/funky"
TOOL_NAME="funky"
TOOL_TEST="funky --version"

fail() {
  echo -e "asdf-${TOOL_NAME}: $*" >&2
  exit 1
}

curl_opts=(-fsSL)
if [[ -n "${GITHUB_API_TOKEN:-}" ]]; then
  curl_opts=(-H "Authorization: token ${GITHUB_API_TOKEN}" "${curl_opts[@]}")
fi

sort_versions() {
  sed 'h; s/[+-]/./g; s/.p\([[:digit:]]\)/.z\1/; s/$/.z/; G; s/\n/ /' |
    LC_ALL=C sort -t. -k 1,1 -k 2,2n -k 3,3n -k 4,4n -k 5,5n | awk '{print $2}'
}

list_github_tags() {
  git ls-remote --tags --refs "${GH_REPO}" |
    grep -o 'refs/tags/.*' | cut -d/ -f3- |
    sed 's/^v//'
}

list_all_versions() {
  list_github_tags
}

get_platform() {
  case "$(uname -s)" in
    Darwin) echo "macos" ;;
    Linux) echo "linux" ;;
    *) fail "Unsupported platform: $(uname -s)" ;;
  esac
}

get_arch() {
  case "$(uname -m)" in
    x86_64 | amd64) echo "x64" ;;
    aarch64 | arm64) echo "aarch64" ;;
    i686 | i386) echo "x86" ;;
    *) fail "Unsupported architecture: $(uname -m)" ;;
  esac
}

get_download_url() {
  local version="$1"
  local platform
  local arch
  platform="$(get_platform)"
  arch="$(get_arch)"
  echo "${GH_REPO}/releases/download/v${version}/funky-v${version}-${platform}-${arch}.zip"
}

download_release() {
  local version="$1"
  local download_path="$2"
  local url
  url="$(get_download_url "$version")"

  echo "* Downloading ${TOOL_NAME} ${version}..."
  if ! curl "${curl_opts[@]}" -o "${download_path}/${TOOL_NAME}.zip" "${url}"; then
    fail "Could not download ${url}"
  fi
}

install_version() {
  local version="$1"
  local install_path="$2"
  local download_path="$3"

  local bin_path="${install_path}/bin"
  mkdir -p "${bin_path}"

  # Find the binary in the extracted download directory
  local binary
  binary="$(find "${download_path}" -name "${TOOL_NAME}" -type f | head -1)"
  if [[ -z "${binary}" ]]; then
    fail "Could not find ${TOOL_NAME} binary in extracted archive"
  fi

  mv "${binary}" "${bin_path}/${TOOL_NAME}"
  chmod +x "${bin_path}/${TOOL_NAME}"

  if [[ ! -x "${bin_path}/${TOOL_NAME}" ]]; then
    fail "Expected ${bin_path}/${TOOL_NAME} to be executable"
  fi

  if ! "${bin_path}/${TOOL_NAME}" --version > /dev/null 2>&1; then
    fail "Could not verify ${TOOL_NAME} installation"
  fi

  echo "* ${TOOL_NAME} ${version} installation was successful!"
}
