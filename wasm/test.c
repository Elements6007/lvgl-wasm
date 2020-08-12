/* Sample App:
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

#ifdef NOTUSED
[luppy@pinebook lvgl-wasm]$ emcc wasm/test.c -s WASM=1 -o wasm/test.html
shared:INFO: EM_IGNORE_SANITY set, ignoring sanity checks
cache:INFO: generating system asset: generated_struct_info.json... (this will be cached in "/home/luppy/.emscripten_cache/wasm/generated_struct_info.json" for subsequent builds)
Fatal: Module::addExport: stackSave already exists
emcc: error: '/usr/bin/wasm-emscripten-finalize --detect-features --global-base=1024 --check-stack-overflow /tmp/emscripten_temp_ss2n_af6/tmpyakvy0aa.wasm -o /tmp/emscripten_temp_ss2n_af6/tmpyakvy0aa.wasm.o.wasm' failed (1)
FAIL: Compilation failed!: ['/usr/lib/emscripten/emcc', '-D_GNU_SOURCE', '-o', '/tmp/tmpyakvy0aa.js', '/tmp/tmphcwy4r4h.c', '-O0', '--js-opts', '0', '--memory-init-file', '0', '-Werror', '-Wno-format', '-s', 'BOOTSTRAPPING_STRUCT_INFO=1', '-s', 'WARN_ON_UNDEFINED_SYMBOLS=0', '-s', 'STRICT=1', '-s', 'SINGLE_FILE=1']
#endif
