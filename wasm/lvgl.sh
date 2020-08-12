# Build LVGL for WebAssembly

# Stop the script on error, echo all commands
set -e -x

# Test Build
# emcc -c -o lv_group.o ././src/lv_core/lv_group.c -g -I src/lv_core -D LV_USE_DEMO_WIDGETS -s WASM=1

# Test Output
wasm-objdump -h wasm/lvgl.o

####
exit

# Build app
make -j 5
