# Build LVGL for WebAssembly

# Stop the script on error, echo all commands
set -e -x

# Test
emcc -c -o lv_group.o ././src/lv_core/lv_group.c -g -I src/lv_core -D LV_USE_DEMO_WIDGETS -s WASM=1

####
exit

# Build app
make
