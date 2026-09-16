#!/usr/bin/env bash
# wasm32 verification script for wasm4games.
#
# Leg 1: build the no_std core to wasm32 (compile-level portability).
# Leg 2: build the C-ABI surface as a cdylib to wasm32 (exports w4g_corpus_digest /
#        w4g_golden_corpus_digest alongside the kernel dispatch).
# Leg 3: EXECUTE the corpus-digest proof under a wasm runtime when one is available:
#        the wasm32-executed digest must equal the native Rust golden — the same oracle
#        the C portability proof (crates/wasm4games-capi/portability_proof.sh) asserts.
#
# Without wasmtime the script degrades honestly to build-only verification
# (RESULT: SKIPPED_NO_RUNTIME); it never launders a build into an execution pass.
set -euo pipefail
cd "$(dirname "$0")/.."   # workspace root

echo "[1/3] Building wasm32 core (no-default-features)..."
cargo build -p wasm4games --target wasm32-unknown-unknown --no-default-features
echo "BUILD: VERIFIED"

echo "[2/3] Building wasm32 C-ABI cdylib (digest exports)..."
cargo build -p wasm4games-capi --target wasm32-unknown-unknown --release
WASM_FILE="target/wasm32-unknown-unknown/release/wasm4games_capi.wasm"
if [ ! -f "$WASM_FILE" ]; then
    echo "ERROR: $WASM_FILE not found"
    exit 1
fi
echo "BUILD: VERIFIED"

echo "[3/3] Runtime execution check..."
if ! command -v wasmtime &>/dev/null; then
    if command -v wasmer &>/dev/null; then
        echo "RESULT: SKIPPED_WASMER_NO_INVOKE — digest invocation is wasmtime-shaped; build legs verified"
    else
        echo "BUILD: VERIFIED — runtime execution requires wasmtime"
        echo "RESULT: SKIPPED_NO_RUNTIME"
    fi
    exit 0
fi

# `--invoke` prints each function's return value; warnings on stderr are suppressed so
# the captured values stay parseable.
COUNT="$(wasmtime run --invoke w4g_pattern_count "$WASM_FILE" 2>/dev/null)"
CORPUS="$(wasmtime run --invoke w4g_corpus_digest "$WASM_FILE" 2>/dev/null)"
GOLDEN="$(wasmtime run --invoke w4g_golden_corpus_digest "$WASM_FILE" 2>/dev/null)"

echo "pattern_count = $COUNT                     # whatever PATTERN_REGISTRY.len() reports (currently 75)"
echo "corpus_digest = $CORPUS (wasm32 execution via wasmtime)"
echo "golden_digest = $GOLDEN (native Rust oracle)"
if [ "$CORPUS" != "$GOLDEN" ]; then
    echo "PORTABILITY_FAIL: wasm32 execution digest diverges from native golden"
    exit 1
fi
echo "PORTABILITY_OK: wasm32 execution reproduces the native golden digest"
echo "RESULT: VERIFIED"
