/* Sample App:
emcc wasm/test_rust.c -s WASM=1 -o wasm/test_rust.html rust/pkg/lvgl_wasm_rust_bg.wasm
*/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>


//  Defined in rust/src/lib.rs
int test_rust(void);

////////////////////////////////////////////////////////////////////
//  Main

int main(int argc, char **argv) {    
    puts("Hello world");
    int i = test_rust();
    printf("test_rust() returned %d\n", i);
    return 0;
}
