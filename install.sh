#!/usr/bin/env bash
set -euo pipefail

REPO="DanielD2G/BuddyTypeCLI"
BINARY="buddytype"
INSTALL_DIR="/usr/local/bin"

# Detect OS
case "$(uname -s)" in
  Linux*)  OS="unknown-linux-gnu" ;;
  Darwin*) OS="apple-darwin" ;;
  *)       echo "Error: unsupported OS '$(uname -s)'" >&2; exit 1 ;;
esac

# Detect architecture
case "$(uname -m)" in
  x86_64|amd64)  ARCH="x86_64" ;;
  arm64|aarch64) ARCH="aarch64" ;;
  *)             echo "Error: unsupported architecture '$(uname -m)'" >&2; exit 1 ;;
esac

TARGET="${ARCH}-${OS}"
ASSET="${BINARY}-${TARGET}.tar.gz"

# Get latest release tag
echo "Fetching latest release..."
TAG=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | cut -d'"' -f4)

if [ -z "$TAG" ]; then
  echo "Error: could not determine latest release" >&2
  exit 1
fi

URL="https://github.com/${REPO}/releases/download/${TAG}/${ASSET}"

echo "Downloading ${BINARY} ${TAG} for ${TARGET}..."

# Download and extract to temp directory
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

curl -fsSL "$URL" -o "${TMP}/${ASSET}"
tar xzf "${TMP}/${ASSET}" -C "$TMP"

# Install
if [ -w "$INSTALL_DIR" ]; then
  mv "${TMP}/${BINARY}" "${INSTALL_DIR}/${BINARY}"
else
  echo "Installing to ${INSTALL_DIR} (requires sudo)..."
  sudo mv "${TMP}/${BINARY}" "${INSTALL_DIR}/${BINARY}"
fi

chmod +x "${INSTALL_DIR}/${BINARY}"

echo ""
echo "buddytype ${TAG} installed to ${INSTALL_DIR}/${BINARY}"
echo "Run 'buddytype' to start."
