/* Sample app to test C and Rust WebAssembly Interoperability
Build sample Rust app: wasm/test_rust.html, test_rust.js, test_rust.wasm
emcc \
    -g \
    wasm/test_rust.c \
    -s WASM=1 \
    -s "EXPORTED_FUNCTIONS=[ '_main', '_test_c', '_test_c_set_buffer', '_test_c_get_buffer', '_test_c_buffer_address', '_test_rust', '_test_rust2', '_test_rust3', '_test_rust_set_buffer', '_test_rust_get_buffer' ]" \
    -o wasm/test_rust.html \
    target/wasm32-unknown-emscripten/debug/liblvgl_wasm_rust.a
*/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <assert.h>
#include "../lvgl.h"

////////////////////////////////////////////////////////////////////
//  Device and Display Buffers

///  RGBA Display Buffer for HTML Canvas
#define DISPLAY_BYTES_PER_PIXEL 4
uint8_t display_buffer[LV_HOR_RES_MAX * LV_VER_RES_MAX * DISPLAY_BYTES_PER_PIXEL];

///  Plot a pixel on the PineTime Device Buffer
void put_device_px(uint16_t x, uint16_t y, uint8_t r, uint8_t g, uint8_t b, uint8_t a) {
    assert(x >= 0); assert(x < LV_HOR_RES_MAX);
    assert(y >= 0); assert(y < LV_VER_RES_MAX);
    //  TODO: Map RGB565 to RGBA
    int i = (y * LV_HOR_RES_MAX * DISPLAY_BYTES_PER_PIXEL) + (x * DISPLAY_BYTES_PER_PIXEL);
    display_buffer[i++] = r;  //  Red
    display_buffer[i++] = g;  //  Green
    display_buffer[i++] = b;  //  Blue
    display_buffer[i++] = a;  //  Alpha
}

///  Return the WebAssembly Address of the Display Buffer
unsigned get_display_buffer(void) {
    uint8_t *p = &display_buffer[0];
    return (unsigned) p;
}

///  Return the width of the Display Buffer
unsigned get_display_width(void) { return LV_HOR_RES_MAX; }

///  Return the height of the Display Buffer
unsigned get_display_height(void) { return LV_VER_RES_MAX; }

///  Render a box to the Display Buffer
int test_display(void) {
    puts("In C: test_display()");
    for (uint16_t x = 0; x < 20; x++) {
        for (uint16_t y = 0; y < 20; y++) {     
            uint8_t r = x % 0xff;
            uint8_t g = y % 0xff;
            uint8_t b = ((x + y) / 2) % 0xff;
            uint8_t a = 0xff;
            put_device_px(x, y, r, g, b, a);
        }
    }
    return 0;
}

/*
///  RGB565 Device Buffer for PineTime
#define DEVICE_BYTES_PER_PIXEL 3  //  TODO: Switch to RGB565
uint8_t device_buffer[LV_HOR_RES_MAX * LV_VER_RES_MAX * DEVICE_BYTES_PER_PIXEL];

///  Plot a pixel on the PineTime Device Buffer
void put_device_px(uint16_t x, uint16_t y, uint8_t r, uint8_t g, uint8_t b, uint8_t a) {
    assert(x >= 0); assert(x < LV_HOR_RES_MAX);
    assert(y >= 0); assert(y < LV_VER_RES_MAX);
    int i = (y * LV_HOR_RES_MAX * DEVICE_BYTES_PER_PIXEL) + (x * DEVICE_BYTES_PER_PIXEL);
    //  TODO: Switch to RGB565
    device_buffer[i++] = r;  //  Red
    device_buffer[i++] = g;  //  Green
    device_buffer[i++] = b;  //  Blue
}
*/

////////////////////////////////////////////////////////////////////
//  Test

//  Defined in rust/src/lib.rs
int test_rust(void);

//  Export memory buffer
uint8_t test_rust_buffer[32] = "Test Rust Buffer";

////////////////////////////////////////////////////////////////////
//  Main

int main(int argc, char **argv) {    
    puts("In C: main()");
    int i = test_rust();
    printf("In C: test_rust() returned %d\n", i);
    uint8_t *p = &test_rust_buffer[0];
    printf("In C: test_rust_buffer is 0x%x\n", (unsigned) p);
    return 0;
}

int test_c(void) {    
    puts("In C: test_c()");
    return 2407;
}

int test_c_set_buffer(void) {
    int i = test_rust_buffer[0];
    test_rust_buffer[0] = 'A';
    return i;
}

int test_c_get_buffer(void) {
    return test_rust_buffer[0];
}

unsigned test_c_buffer_address(void) {
    uint8_t *p = &test_rust_buffer[0];
    return (unsigned) p;
}