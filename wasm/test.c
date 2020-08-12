/*
emcc wasm/test.c -s WASM=1 -o wasm/test.html
*/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

////////////////////////////////////////////////////////////////////
//  Main

int main(int argc, char **argv) {
    puts("Hello world");
    return 0;
}
