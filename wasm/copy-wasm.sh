#!/usr/bin/env bash
# Copy outputs to docs/lvgl.js, lvgl.wasm, script.js, script.wasm

# Stop the script on error, echo all commands
set -e -x

if [ -e wasm/lvgl.wasm ]; then
    cp wasm/lvgl.wasm docs/lvgl.wasm
fi

if [ -e wasm/lvgl.js ]; then
    cp wasm/lvgl.js docs/lvgl.js
fi

if [ -e wasm/script.wasm ]; then
    cp wasm/script.wasm docs/script.wasm
fi

if [ -e wasm/script.js ]; then
    cp wasm/script.js docs/script.js
fi
