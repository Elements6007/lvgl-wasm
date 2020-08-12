# Build LVGL for WebAssembly
# Install emscripten on macOS: 
# brew install emscripten
# brew install binaryen
# (upgrade llvm to 10.0.0)
# brew upgrade llvm
# nano /usr/local/Cellar/emscripten/1.40.1/libexec/.emscripten
# Change BINARYEN_ROOT to 
# BINARYEN_ROOT = os.path.expanduser(os.getenv('BINARYEN', '/usr/local')) # directory

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
    -Wl,-Map=wasm/lvgl.map \
    -L/usr/lib/aarch64-linux-gnu/mesa-egl \
    -lGLESv2

MAINS  := $(addsuffix .o, $(TARGETS) )
OBJ    := \
	$(MAINS) \
	$(CSRCS:.c=.o) \
	$(WASM_CSRCS:.c=.o)

.PHONY: all clean

all: $(TARGETS)

clean:
	rm -f $(TARGETS) $(OBJ)

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
