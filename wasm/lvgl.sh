#!/usr/bin/env bash
# Build LVGL for WebAssembly

# Stop the script on error, echo all commands
set -e -x

# Build LVGL app: wasm/lvgl.html, lvgl.js, lvgl.wasm
make -j

# Build sample app: wasm/test.html, test.js, test.wasm
emcc wasm/test.c -s WASM=1 -o wasm/test.html

#####
# setup - only needed once
rustup default nightly
rustup target add wasm32-unknown-emscripten

# Build Rust modules
cargo build --target=wasm32-unknown-emscripten

# Install WebAssembly builder for Rust
# cargo install wasm-pack

# pushd rust
# wasm-pack build
# popd

# Build sample Rust app: wasm/test_rust.html, test_rust.js, test_rust.wasm
emcc wasm/test_rust.c -s WASM=1 -s LLD_REPORT_UNDEFINED -o wasm/test_rust.html target/wasm32-unknown-emscripten/debug/liblvgl_wasm_rust.a

# Test Compile
# emcc -c -o lv_group.o ././src/lv_core/lv_group.c -g -I src/lv_core -D LV_USE_DEMO_WIDGETS -s WASM=1

# Test Output
# wasm-objdump -h wasm/lvgl.o
