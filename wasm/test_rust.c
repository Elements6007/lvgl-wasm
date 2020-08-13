/* Sample App:
emcc wasm/test_rust.c -s WASM=1 -o wasm/test_rust.html rust/pkg/lvgl_wasm_rust_bg.wasm
*/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <assert.h>

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