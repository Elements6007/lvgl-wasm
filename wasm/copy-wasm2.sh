#!/usr/bin/env bash
# Copy outputs to docs/lvgl2.js, lvgl2.wasm

# Stop the script on error, echo all commands
set -e -x

cp wasm/lvgl.wasm docs/lvgl2.wasm

# Change lvgl.wasm to lvgl2.wasm
cat wasm/lvgl.js \
    | sed 's/lvgl.wasm/lvgl2.wasm/' \
    >docs/lvgl2.js
