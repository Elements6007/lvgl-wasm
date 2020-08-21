#!/usr/bin/env bash
# Copy outputs to docs/rust.js, rust.wasm

# Stop the script on error, echo all commands
set -e -x

cp wasm/lvgl.wasm docs/rust.wasm

# Change lvgl.wasm to rust.wasm
cat wasm/lvgl.js \
    | sed 's/lvgl.wasm/rust.wasm/' \
    >docs/rust.js
