#!/bin/bash

CARGO_TARGET_DIR=wasm/ cargo build --release --target wasm32-unknown-unknown

# Optimize for size.
wasm-opt -Os -o web/quadtris.wasm wasm/wasm32-unknown-unknown/release/quadtris.wasm
cp -r res web/