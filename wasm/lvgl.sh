#!/usr/bin/env bash
# Build LVGL for WebAssembly

# Stop the script on error, echo all commands
set -e -x

# Rewrite WatchFaceDigital.cpp to build with WebAssembly:
# Change <libs/date/includes/date/date.h>
#   To "date.h"
# Change <Components/DateTime/DateTimeController.h>
#   To "DateTimeController.h"
# Change <libs/lvgl/lvgl.h>
#   To "../lvgl.h"
# Change "../DisplayApp.h"
#   To "DisplayApp.h"
# Change obj->user_data
#   To backgroundLabel_user_data
# Change backgroundLabel->user_data
#   To backgroundLabel_user_data
# Remove Screen(app),
cat clock/WatchFaceDigital.cpp \
    | sed 's/<libs\/date\/includes\/date\/date.h>/"date.h"/' \
    | sed 's/<Components\/DateTime\/DateTimeController.h>/"DateTimeController.h"/' \
    | sed 's/<libs\/lvgl\/lvgl.h>/"..\/lvgl.h"/' \
    | sed 's/"..\/DisplayApp.h"/"DisplayApp.h"/' \
    | sed 's/obj->user_data/backgroundLabel_user_data/' \
    | sed 's/backgroundLabel->user_data/backgroundLabel_user_data/' \
    | sed 's/Screen(app),//' \
    >clock/ClockTmp.cpp

# Build LVGL app: wasm/lvgl.html, lvgl.js, lvgl.wasm
make -j

# Build Rust modules with emscripten compatibility
# cargo build --target=wasm32-unknown-emscripten

# Build sample Rust app: wasm/test_rust.html, test_rust.js, test_rust.wasm
# emcc \
#     -g \
#     wasm/test_rust.c \
#     -s WASM=1 \
#     -s "EXPORTED_FUNCTIONS=[ '_main', '_get_display_buffer', '_get_display_width', '_get_display_height', '_test_display', '_test_c', '_test_c_set_buffer', '_test_c_get_buffer', '_test_c_buffer_address', '_test_rust', '_test_rust2', '_test_rust3', '_test_rust_set_buffer', '_test_rust_get_buffer' ]" \
#     -o wasm/test_rust.html \
# 	-I src/lv_core \
#     target/wasm32-unknown-emscripten/debug/liblvgl_wasm_rust.a

# Build sample app: wasm/test.html, test.js, test.wasm
# emcc \
#     -g \
#     wasm/test.c \
#     -s WASM=1 \
#     -o wasm/test.html

# Dump the WebAssembly modules
wasm-objdump -x wasm/lvgl.wasm >wasm/lvgl.txt
# wasm-objdump -x wasm/test.wasm >wasm/test.txt
# wasm-objdump -x wasm/test_rust.wasm >wasm/test_rust.txt

# Rename the HTML files so we don't overwrite the updates
mv wasm/lvgl.html wasm/lvgl.old.html
# mv wasm/test.html wasm/test.old.html
# mv wasm/test_rust.html wasm/test_rust.old.html
