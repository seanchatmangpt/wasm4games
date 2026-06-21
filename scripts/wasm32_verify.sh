#!/usr/bin/env bash
# wasm32 verification script for wasm4games.
# Builds the wasm32 target and attempts runtime execution if a runtime is available.
set -euo pipefail
cd "$(dirname "$0")/.."   # workspace root

echo "[1/2] Building wasm32 target (no-default-features)..."
cargo build -p wasm4games --target wasm32-unknown-unknown --no-default-features
echo "BUILD: VERIFIED"

echo "[2/2] Checking for wasm runtime..."
if command -v wasmtime &>/dev/null; then
    RUNTIME="wasmtime"
elif command -v wasmer &>/dev/null; then
    RUNTIME="wasmer"
else
    RUNTIME=""
fi

if [ -z "$RUNTIME" ]; then
    echo "BUILD: VERIFIED — runtime execution requires wasmtime or wasmer"
    echo "RESULT: SKIPPED_NO_RUNTIME"
    exit 0
fi

WASM_FILE="$(find target/wasm32-unknown-unknown -name 'wasm4games*.wasm' | head -1)"
if [ -z "$WASM_FILE" ]; then
    echo "ERROR: No .wasm artifact found under target/wasm32-unknown-unknown/"
    exit 1
fi

echo "Running $WASM_FILE via $RUNTIME..."
$RUNTIME "$WASM_FILE"
echo "RESULT: VERIFIED"
