#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

echo "Building WASM plugin..."
cargo build --release --target wasm32-unknown-unknown -p dgs-wasm

echo "WASM binary built at: target/wasm32-unknown-unknown/release/dgs_wasm.wasm"

echo "Testing with example..."
typst compile --root . examples/demo.typ examples/demo.pdf

echo "Done! Output: examples/demo.pdf"
