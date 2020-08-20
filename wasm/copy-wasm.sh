#!/usr/bin/env bash
# Copy outputs to docs/lvgl.js, lvgl.wasm

# Stop the script on error, echo all commands
set -e -x

cp wasm/lvgl.wasm docs/lvgl.wasm
cp wasm/lvgl.js docs/lvgl.js
