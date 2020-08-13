#!/usr/bin/env bash
# Build LVGL for WebAssembly

# Stop the script on error, echo all commands
set -e -x

# Install WebAssembly builder for Rust
cargo install wasm-pack

# Build Rust modules
pushd rust
wasm-pack build
popd

# Build LVGL app: wasm/lvgl.html, lvgl.js, lvgl.wasm
make -j

# Build sample app: wasm/test.html, test.js, test.wasm
emcc wasm/test.c -s WASM=1 -o wasm/test.html

# Test Compile
# emcc -c -o lv_group.o ././src/lv_core/lv_group.c -g -I src/lv_core -D LV_USE_DEMO_WIDGETS -s WASM=1

# Test Output
# wasm-objdump -h wasm/lvgl.o
