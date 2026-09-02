#!/usr/bin/env bash
set -euo pipefail

# Copies package files into the typst/packages clone for PR submission.
# Set TYPST_PACKAGES env var to override the default path.

cd "$(dirname "$0")"

TYPST_PACKAGES="${TYPST_PACKAGES:-../typst_packages}"
PKG_DIR="$TYPST_PACKAGES/packages/preview/dgs/0.0.1"

echo "=== Packaging dgs into $PKG_DIR ==="

# Remove previous contents to avoid leftover files
rm -rf "$PKG_DIR"
mkdir -p "$PKG_DIR/src" "$PKG_DIR/wasm"

# Copy package files
cp lib.typ typst.toml README.md LICENSE "$PKG_DIR/"
cp src/*.typ "$PKG_DIR/src/"
cp wasm/dgs_wasm.wasm "$PKG_DIR/wasm/"

echo "=== Done! Package contents: ==="
find "$PKG_DIR" -type f | sort
