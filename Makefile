# Build LVGL for WebAssembly

# Install emscripted on Manjaro:
# sudo pacman -S emscripten
# sudo pacman -S wabt
# source /etc/profile.d/emscripten.sh
# rm -rf ~/.emscripten_cache
# make

# Shows error:
# emcc: error: unexpected binaryen version: 95 (expected 93) [-Wversion-check] [-Werror]
# FAIL: Compilation failed!: ['/usr/lib/emscripten/emcc', '-D_GNU_SOURCE', '-o', '/tmp/tmpbe4ik5na.js', '/tmp/tmpzu5jusdg.c', '-O0', '--js-opts', '0', '--memory-init-file', '0', '-Werror', '-Wno-format', '-s', 'BOOTSTRAPPING_STRUCT_INFO=1', '-s', 'WARN_ON_UNDEFINED_SYMBOLS=0', '-s', 'STRICT=1', '-s', 'SINGLE_FILE=1']
# make: *** [Makefile:66: wasm/lvgl] Error 1

# Install emscripten on macOS: 
# brew install emscripten
# brew install binaryen
# (upgrade llvm to 10.0.0)
# brew upgrade llvm
# nano /usr/local/Cellar/emscripten/1.40.1/libexec/.emscripten
# Change BINARYEN_ROOT and LLVM_ROOT to 
# BINARYEN_ROOT = os.path.expanduser(os.getenv('BINARYEN', '/usr/local')) # directory
# LLVM_ROOT = os.path.expanduser(os.getenv('LLVM', '/usr/local/opt/llvm/bin')) # directory

# Previously: 
# LLVM_ROOT = os.path.expanduser(os.getenv('LLVM', '/usr/local/opt/llvm@3.9/bin')) # directory

# Compile to WebAssembly:
# emcc hello.c -s WASM=1 -o hello.html

# Define $(CSRCS)
LVGL_DIR 	  := .
LVGL_DIR_NAME := .
include lvgl.mk

WASM_CSRCS := \
	demo/lv_demo_widgets.c \
	wasm/lv_port_disp.c

TARGETS:= wasm/lvgl

DEPS   := lv_conf.h

CC     := emcc

CCFLAGS := \
	-g \
	-I src/lv_core \
	-D LV_USE_DEMO_WIDGETS \
	-s WASM=1

LDFLAGS := \
    -Wl,-Map=wasm/lvgl.map

MAINS  := $(addsuffix .o, $(TARGETS) )
OBJ    := \
	$(MAINS) \
	$(CSRCS:.c=.o) \
	$(WASM_CSRCS:.c=.o)

.PHONY: all clean

all: $(TARGETS)

clean:
	rm -f $(TARGETS) $(OBJ)
	rm -rf $(HOME)/.emscripten_cache

$(OBJ): %.o : %.c $(DEPS)
	$(CC) -c -o $@ $< $(CCFLAGS)

$(TARGETS): % : $(filter-out $(MAINS), $(OBJ)) %.o
	$(CC) -o $@ \
	-Wl,--start-group \
	$(LIBS) \
	$^ \
	-Wl,--end-group \
	$(CCFLAGS) \
	$(LDFLAGS)
