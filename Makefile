# Build LVGL for WebAssembly: wasm/lvgl.html, lvgl.js, lvgl.wasm

###############################################################################
# Typical Compile to WebAssembly with emscripten
# emcc hello.c -s WASM=1 -o hello.html

# Define $(CSRCS)
LVGL_DIR 	  := .
LVGL_DIR_NAME := .
include lvgl.mk

# WebAssembly C and C++ Source Files
WASM_CSRCS := \
	demo/lv_demo_widgets.c \
	wasm/lv_port_disp.c \
	clock/BleController.cpp \
	clock/Clock.cpp \
	clock/ClockHelper.cpp \
	clock/DateTimeController.cpp \
	clock/LittleVgl.cpp \
	clock/Symbols.cpp

# Build LVGL app: wasm/lvgl.html, lvgl.js, lvgl.wasm
TARGETS:= wasm/lvgl

DEPS   := lv_conf.h

# Use emscripten compiler
CC     := emcc
CPP    := em++

# Options for emscripten. We specify the C and Rust WebAssembly functions to be exported.
CCFLAGS := \
	-g \
	-I src/lv_core \
	-D LV_USE_DEMO_WIDGETS \
	-s WASM=1 \
    -s "EXPORTED_FUNCTIONS=[ '_main', '_get_display_buffer', '_get_display_width', '_get_display_height', '_test_display', '_init_display', '_render_display', '_render_widgets', '_create_clock', '_refresh_clock', '_update_clock' ]"

LDFLAGS :=

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

# TODO: Build C++ files with em++
# $(OBJ): %.o : %.cpp $(DEPS)
#	$(CPP) -c -o $@ $< $(CCFLAGS)

$(TARGETS): % : $(filter-out $(MAINS), $(OBJ)) %.o
	$(CC) -o $@.html \
	-Wl,--start-group \
	$(LIBS) \
	$^ \
	-Wl,--end-group \
	$(CCFLAGS) \
	$(LDFLAGS)
