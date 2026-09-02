#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

echo "=== Building WASM plugin ==="
cargo build --release --target wasm32-unknown-unknown -p dgs-wasm

echo "=== Copying WASM binary to wasm/ ==="
cp target/wasm32-unknown-unknown/release/dgs_wasm.wasm wasm/

echo "=== Running tests ==="
cargo test

echo "=== Compiling example ==="
typst compile --root . examples/demo.typ examples/demo.pdf

echo "=== Done! ==="
echo "  WASM: wasm/dgs_wasm.wasm"
echo "  PDF:  examples/demo.pdf"
