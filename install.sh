#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
PKG="dgs"
VER="$(grep '^version' typst.toml | cut -d'"' -f2)"
DATA_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/typst/packages/preview/$PKG/$VER"
CACHE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/typst/packages/preview/$PKG/$VER"
mkdir -p "$DATA_DIR/src" "$DATA_DIR/wasm" "$DATA_DIR/assets"
mkdir -p "$CACHE_DIR/src" "$CACHE_DIR/wasm" "$CACHE_DIR/assets"
cp lib.typ typst.toml README.md LICENSE "$DATA_DIR/" 2>/dev/null || true
cp src/*.typ "$DATA_DIR/src/"
cp wasm/dgs_wasm.wasm "$DATA_DIR/wasm/"
cp assets/readme.png "$DATA_DIR/assets/" 2>/dev/null || true
cp -a "$DATA_DIR/." "$CACHE_DIR/"
echo "Installed $PKG:$VER to $DATA_DIR and $CACHE_DIR"
