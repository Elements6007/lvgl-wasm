<h1 align="center"> WebAssembly Simulator for Rust on Mynewt with LVGL  </h1>

![WebAssembly Simulator for Rust on Mynewt](https://lupyuen.github.io/images/rust-on-riot-simulator.png)

__Note: This is the `mynewt` branch that supports Watch Faces built with Rust on Mynewt__

__Simulate Rust on Mynewt Watch Face__ in Web Browser (with WebAssembly), for easier development of custom watch faces

- [Online Rust on Mynewt Demo](https://lupyuen.github.io/barebones-watchface/lvgl.html)

- [Watch Face Source Code for Rust on Mynewt](https://github.com/lupyuen/barebones-watchface/blob/master/src/lib.rs)

- [Watch Face Crate on crates.io](https://crates.io/crates/barebones-watchface)

- [GitHub Actions Workflow for Simulator](https://github.com/lupyuen/barebones-watchface/blob/master/.github/workflows/simulator.yml)

# Features

1. __Compiles actual Rust On Mynewt Watch Face__ from Rust to WebAssembly without any code changes

1. __Uses GitHub Actions Workflow__ to build any fork of Rust On Mynewt into WebAssembly

1. __Renders LVGL to HTML Canvas__ directly via WebAssembly, without using SDL2. See [`lvgl.html`](docs/lvgl.html#L1296-L1357)

1. __Supports RGB565 Framebuffer Format__ used by PineTime Display Controller, so that bitmaps will be rendered correctly. [Custom Bitmap Demo](https://appkaki.github.io/lvgl-wasm/lvgl2.html) / [Source Code](clock/Clock2.cpp)

1. __Shows current date and time__

# Upcoming Features

1. __Support Custom Fonts and Symbols__ for LVGL

1. __Accept Touch Input__ for LVGL

1. Allow Watch Faces to be __built online in Rust with online preview__

# References

- ["Bluetooth Time Sync, Rust Watch Faces and LVGL on PineTime Mynewt"](https://lupyuen.github.io/pinetime-rust-mynewt/articles/timesync)

- ["Preview PineTime Watch Faces in your Web Browser with WebAssembly"](https://lupyuen.github.io/pinetime-rust-mynewt/articles/simulator)

# How To Build The Simulator

TODO

To build PineTime Watch Face Simulator on Linux x64 or Arm64...

1.  Install emscripten and wabt. See instructions below.

1.  Install Rust from `rustup.rs`

1.  Enter...

    ```bash
    git clone --branch rust  https://github.com/AppKaki/lvgl-wasm
    cd lvgl-wasm
    ```

1.  Build the Rust app...

    ```bash
    rustup default nightly
    rustup target add wasm32-unknown-emscripten
    cargo build
    ```

    Ignore this error, we'll fix in the next step...

    ```
    error[E0432]: unresolved import `ad`
    --> /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/cty-0.1.5/src/lib.rs:8:9
    pub use ad::*;
    ^^ maybe a missing crate `ad`?
    ```

1.  Change `$HOME/.cargo/registry/src/github.com-*/cty-0.1.5/src/lib.rs`

    From...

    ```
    target_arch = "arm"
    ```

    To...

    ```
    target_arch = "arm", target_arch = "wasm32"
    ```

    ```bash
    cat $HOME/.cargo/registry/src/github.com-*/cty-0.1.5/src/lib.rs \
        | sed 's/target_arch = "arm"/target_arch = "arm", target_arch = "wasm32"/' \
        >/tmp/lib.rs
    cp /tmp/lib.rs $HOME/.cargo/registry/src/github.com-*/cty-0.1.5/src/lib.rs
    rm /tmp/lib.rs
    ```
        
1.  Copy Rust Watch Face from our fork of pinetime-mynewt-rust into lvgl-wasm...

    ```bash
    rm -r rust/app
    # Assume that our fork of pinetime-mynewt-rust is at ~/pinetime-rust-riot
    cp -r ~/pinetime-rust-riot/rust/app rust
    ```

    This is the Rust On RIOT Watch Face that will be built into the Simulator.

1.  Change `rust/app/Cargo.toml`

    From...

    ```
    crate-type = ["staticlib"]
    ```

    To...

    ```
    crate-type = ["lib"]
    ```

    ```bash
    cat rust/app/Cargo.toml \
        | sed 's/crate-type = \["staticlib"\]/crate-type = \["lib"\]/' \
        >rust/app/Cargo.toml.new
    cp rust/app/Cargo.toml.new rust/app/Cargo.toml
    rm rust/app/Cargo.toml.new
    ```

1.  Build the LVGL WebAssembly app containing our Watch Face...

    ```bash
    cargo build
    make -f rust/Makefile
    ```

1.  Copy the generated WebAssembly files to the `docs` folder (used by GitHub Pages)...

    ```bash
    cp wasm/lvgl.js wasm/lvgl.wasm docs
    ```

    We don't need `lvgl.html` because `docs` already contains a version of `lvgl.html` with custom JavaScript.

1.  Start a Web Server for the `docs` folder, because WebAssembly doesn't work when opened from the filesystem.

    __For Arm64:__ Use [`darkhttpd`](https://unix4lyfe.org/darkhttpd/)...

    ```bash
    darkhttpd docs
    ```

    __For x64:__ Use the Chrome Extension [Web Server for Chrome](https://chrome.google.com/webstore/detail/web-server-for-chrome/ofhbbkphhbklhfoeikjpcbhemlocgigb?hl=en) and set the folder to `docs`

1.  Launch a Web Browser and open the URL shown by `darkhttpd` or Web Server for Chrome.

    Enter `lvgl.html` in the URL bar to view the PineTime Watch Face Simulator.

In case of problems, compare with the following...

- [GitHub Actions workflow](https://github.com/lupyuen/pinetime-rust-riot/blob/master/.github/workflows/simulator.yml)

- [GitHub Actions build log](https://github.com/lupyuen/pinetime-rust-riot/actions?query=workflow%3A%22Simulate+PineTime+Firmware%22)

# How It Works

TODO

Rust on RIOT WebAssembly is compiled in multiple parts...

1. Watch Face Library in Rust, from [`rust`](rust)

   Compiled with `cargo` with target `wasm32-unknown-emscripten` into a WebAssembly Static Library: `target/wasm32-unknown-emscripten/debug/libwasm.a`
   
   Contains the Watch Face code [`rust/app`](rust/app), LVGL wrapper [`rust/lvgl`](rust/lvgl), WebAssembly interface [`rust/wasm`](rust/wasm)
   
   Uses macros from [`rust/macros`](rust/macros)

1. LVGL Library in C, from [`src`](src)

   Compiled from C to WebAssembly with [emscripten](https://developer.mozilla.org/en-US/docs/WebAssembly/C_to_wasm)

1. WebAssembly Interface in C, from [`wasm/lvgl.c`](wasm/lvgl.c)

   Compiled from C to WebAssembly with [emscripten](https://developer.mozilla.org/en-US/docs/WebAssembly/C_to_wasm)

The Makefile [`rust/Makefile`](rust/Makefile) links the above into WebAssembly like this...

```bash
emcc -o wasm/lvgl.html \
	-Wl,--start-group \
  target/wasm32-unknown-emscripten/debug/libwasm.a \
	(List of C and C++ object files from LVGL and InfiniTime Sandbox) \
	-Wl,--end-group \
	-g \
	-I src/lv_core \
	-D LV_USE_DEMO_WIDGETS \
	-s WASM=1 \
    -s "EXPORTED_FUNCTIONS=[ '_main', '_get_display_buffer', '_get_display_width', '_get_display_height', '_test_display', '_init_display', '_render_display', '_render_widgets', '_create_clock', '_refresh_clock', '_update_clock' ]"
```

The emscripten compiler `emcc` generates three files in folder `wasm`...

- `lvgl.wasm`: WebAssembly Executable Code, containing our Watch Face, LVGL and the InfiniTime Sandbox. [Sample File](docs/lvgl.wasm)

- `lvgl.js`: Provides the JavaScript glue that's needed to load `lvgl.wasm` and run it in a Web Browser. [Sample File](docs/lvgl.js)

- `lvgl.html`: The HTML file that calls `lvgl.js` to render the user interface.

    We won't be using this file, because we have a [custom version of `lvgl.html`](docs/lvgl.html)

`EXPORTED_FUNCTIONS` are the C functions that will be exposed from WebAssembly to JavaScript. See the section on "Exported Functions" below.

## Rename the HTML files

Because we use a custom `lvgl.html`, we rename the generated `lvgl.html` to prevent overwriting...

```bash
# Rename the HTML files so we don't overwrite the updates
mv wasm/lvgl.html wasm/lvgl.old.html
```

## Dump the WebAssembly modules

For troubleshooting, we may dump the text version of the WebAssembly module to `lvgl.txt`...

```bash
# Dump the WebAssembly modules
wasm-objdump -x wasm/lvgl.wasm >wasm/lvgl.txt
```

[Sample `lvgl.txt`](docs/lvgl.txt)

## Patch for `cty` crate

This error appears because WebAssembly is not defined for the imported C types...

```
error[E0432]: unresolved import `ad`
 --> /home/luppy/.cargo/registry/src/github.com-1ecc6299db9ec823/cty-0.1.5/src/lib.rs:8:9
  |
8 | pub use ad::*;
  |         ^^ maybe a missing crate `ad`?
```

Edit `~/.cargo/registry/src/github.com-*/cty-0.1.5/src/lib.rs`

Under `aarch64`, insert a line for `wasm32`...

```rust
#[cfg(any(target_arch = "aarch64",
          target_arch = "arm",
          target_arch = "asmjs",
          target_arch = "powerpc",
          target_arch = "powerpc64",
          target_arch = "s390x",
          target_arch = "wasm32"))]
mod ad {
    pub type c_char = ::c_uchar;

    pub type c_int = i32;
    pub type c_uint = u32;
}
```

If we see this error...

```
error: language item required, but not found: `eh_personality`
```

Check `Cargo.toml`...

```yaml
[profile.dev]
panic         = "abort"     # Disable stack unwinding on panic
```

# Rust on RIOT Sandbox

TODO

Rust on RIOT Simulator runs in a Web Browser based on WebAssembly (somewhat similar to Java Applets). [More about WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Concepts)

Our Watch Face Module in Rust from [`rust/app`](rust/app) calls functions from two providers...

1. [LVGL UI Toolkit Library](https://docs.lvgl.io/latest/en/html/index.html)

1. [Rust on RIOT](https://github.com/lupyuen/pinetime-rust-riot)

`lvgl-wasm` simulates the minimal set of functions needed for rendering Watch Faces. (RIOT is not supported by the Simulator)

Hence `lvgl-wasm` works like a __Sandbox__.  Here's how the Sandbox works...

## Exported Functions

The Sandbox exports the following WebAssembly functions from C to JavaScript...

### Clock Functions

These functions create the Watch Face from the [`rust/app`](rust/app) module, render the LVGL widgets on the Watch Face, and update the time...

-   `create_clock()`

    Create an instance of the clock. 
    
    From [`clock/ClockHelper.cpp`](clock/ClockHelper.cpp)

-   `refresh_clock()`

    Redraw the clock. 
    
    From [`clock/ClockHelper.cpp`](clock/ClockHelper.cpp)

-   `update_clock(year, month, day, hour, minute, second)`

    Set the current date and time in `DateTimeController`. The time needs to be adjusted for the current timezone, see the JavaScript call to `update_clock()` below.

    From [`clock/ClockHelper.cpp`](clock/ClockHelper.cpp)

    TODO: This code needs to be fixed to show the correct date and time
    
### Display Functions

These functions initialise the LVGL library and render the LVGL Widgets to the WebAssembly Display Buffer...

-   `init_display()`

    Init the LVGL display. 
    
    From [`wasm/lvgl.c`](wasm/lvgl.c)

-   `render_display()`

    Render the LVGL display in 16-bit RGB565 format. From [`wasm/lvgl.c`](wasm/lvgl.c)

    Calls the WebAssembly Display Driver defined in [`wasm/lv_port_disp.c`](wasm/lv_port_disp.c)

    Which calls `put_display_px()` to draw individual pixels to the the WebAssembly Display Buffer: [`wasm/lvgl.c`](wasm/lvgl.c)
    
### Display Buffer Functions

The WebAssembly Display Driver maintains a Display Buffer: 240 x 240 array of pixels, 4 bytes per pixel, in RGBA colour format: [`wasm/lvgl.c`](wasm/lvgl.c)

```c
///  RGBA WebAssembly Display Buffer that will be rendered to HTML Canvas
#define LV_HOR_RES_MAX          240
#define LV_VER_RES_MAX          240
#define DISPLAY_BYTES_PER_PIXEL 4
uint8_t display_buffer[LV_HOR_RES_MAX * LV_VER_RES_MAX * DISPLAY_BYTES_PER_PIXEL];
```

Our JavaScript code copies the Display Buffer from WebAssembly Memory and renders to HTML Canvas by calling the following functions...

-   `get_display_width()`

    Returns 240, the width of the WebAssembly Display Buffer. 
    
    From [`wasm/lvgl.c`](wasm/lvgl.c)

-   `get_display_height()`

    Returns 240, the height of the WebAssembly Display Buffer. 
    
    From [`wasm/lvgl.c`](wasm/lvgl.c)

-   `get_display_buffer()`

    Return the WebAssembly Address of the WebAssembly Display Buffer. 
    
    From [`wasm/lvgl.c`](wasm/lvgl.c)

Note that JavaScript is allowed to read and write to WebAssembly Memory (treating it like a JavaScript array of bytes). But WebAssembly can't access any JavaScript Memory.

That's why we designed the Display Buffer Functions to manipulate WebAssembly Memory.

### Test Functions

For testing only...

-   `test_display()`

    (For Testing) Render a colour box to the WebAssembly Display Buffer.
    
    From [`wasm/lvgl.c`](wasm/lvgl.c)

-   `render_widgets()`

    (For Testing) Render a Button Widget and a Label Widget. 
    
    From [`wasm/lvgl.c`](wasm/lvgl.c)

### Other Functions

-   `main()`: Does nothing. 

    From [`wasm/lvgl.c`](wasm/lvgl.c)

## Sandbox API

The Sandbox simulates Rust on RIOT OS by exposing the following Rust modules to the Watch Face Module in [`rust/app`](rust/app)...

### New Modules

The following Rust modules were created for the Simulator...

- [`rust/wasm`](rust/wasm)

  Exposes the Rust functions `create_clock`, `refresh_clock`, `update_clock` that will be called by the C WebAssembly Functions above.

  See [`rust/wasm/src/lib.rs`](rust/wasm/src/lib.rs)

### Mocked Modules

The following Rust on RIOT modules from were mocked up (i.e. made non-functional) to run in the Simulator...

(None)

### Reused Classes

The following modules were reused from Rust on RIOT with minor changes...

- [`rust/lvgl`](rust/lvgl)

  Safe Wrapper for LVGL. Based on [`pinetime-rust-riot/rust/lvgl`](https://github.com/lupyuen/pinetime-rust-riot/tree/master/rust/lvgl)

- [`rust/macros`](rust/macros) 

  Macros for building Safe Wrappers. Based on [`pinetime-rust-riot/rust/macros`](https://github.com/lupyuen/pinetime-rust-riot/tree/master/rust/macros)

## Sandbox Styles

TODO: Port the LVGL v6 styles from Rust on RIOT to v7 for the Simulator

# Simulator JavaScript

The JavaScript functions here call the Exported WebAssembly Functions to render the Watch Face. From [`docs/lvgl.html`](docs/lvgl.html)

## Initialise WebAssembly

We register a callback in the emscripten API, to be notified when the WebAssembly Module `lvgl.wasm` has been loaded...

```javascript
//  In JavaScript: Wait for emscripten to be initialised
Module.onRuntimeInitialized = function() {
  //  Render LVGL to HTML Canvas
  render_canvas();
};
```

## Initialise LVGL Display

When the WebAssembly Module `lvgl.wasm` has been loaded, we call the WebAssembly Function `init_display()` to initialise the LVGL display...

```javascript
/// In JavaScript: Create the Watch Face in WebAssembly
function render_canvas() {
  //  Init LVGL Display
  Module._init_display();
```

## Create Watch Face

Then we create the LVGL Watch Face Class from `Clock.cpp`...

```javascript
  //  Create the Watch Face in WebAssembly
  Module._create_clock();
```

## Update Watch Face Time

Every minute we update the Watch Face time in `DateTimeController`...

```javascript
/// In JavaScript: Update the Watch Face time in WebAssembly and render the WebAssembly Display Buffer to the HTML Canvas
function updateCanvas() {
  //  Update the WebAssembly Date and Time: year, month, day, hour, minute, second
  const localTime = new Date();
  const timezoneOffset = localTime.getTimezoneOffset();  //  In mins
  //  Compensate for the time zone
  const now = new Date(
    localTime.valueOf()             //  Convert time to millisec
    - (timezoneOffset * 60 * 1000)  //  Convert mins to millisec
  );
  Module._update_clock(
    now.getFullYear(),
    now.getMonth() - 1,  //  getMonth() returns 1 to 12
    now.getDay(), 
    now.getHours(),
    now.getMinutes(),
    now.getSeconds()
  );
```

Note that we need to compensate for the timezone.

## Redraw Watch Face

And redraw the Watch Face in `Clock.cpp`...

```javascript
  //  Update the Watch Face time in WebAssembly
  Module._refresh_clock();
```

## Render LVGL Widgets to WebAssembly Display Buffer

We call LVGL to render the Widgets into the WebAssembly Display Buffer...

```javascript
  //  Render LVGL Widgets to the WebAssembly Display Buffer
  Module._render_display();
```

## Resize HTML Canvas

We resize the HTML Canvas to PineTime's 240 x 240 resolution, scaled by 2 times...

```javascript
  const DISPLAY_SCALE = 2;  //  Scale the canvas width and height

  //  Fetch the PineTime dimensions from WebAssembly Display Buffer
  var width = Module._get_display_width();
  var height = Module._get_display_height();

  //  Resize the canvas to PineTime dimensions (240 x 240)
  if (
    Module.canvas.width != width * DISPLAY_SCALE ||
    Module.canvas.height != height * DISPLAY_SCALE
  ) {
    Module.canvas.width = width * DISPLAY_SCALE;
    Module.canvas.height = height * DISPLAY_SCALE;
  }
```

## Fetch HTML Canvas

We fetch the HTML Canvas...

```javascript
  //  Fetch the canvas pixels
  var ctx = Module.canvas.getContext('2d');
  var imageData = ctx.getImageData(0, 0, width * DISPLAY_SCALE, height * DISPLAY_SCALE);
  var data = imageData.data;
```

## Copy WebAssembly Display Buffer to HTML Canvas

We copy the pixels from the WebAssembly Display Buffer to the HTML Canvas (which uses 24-bit RGBA format)...

```javascript
  const DISPLAY_SCALE = 2;  //  Scale the canvas width and height
  const DISPLAY_BYTES_PER_PIXEL = 4;  //  4 bytes per pixel: RGBA

  //  Copy the pixels from the WebAssembly Display Buffer to the canvas
  var addr = Module._get_display_buffer();
  Module.print(`In JavaScript: get_display_buffer() returned ${toHex(addr)}`);          
  for (var y = 0; y < height; y++) {
    //  Scale the pixels vertically to fill the canvas
    for (var ys = 0; ys < DISPLAY_SCALE; ys++) {
      for (var x = 0; x < width; x++) {
        //  Copy from src to dest with scaling
        const src = ((y * width) + x) * DISPLAY_BYTES_PER_PIXEL;
        const dest = ((((y * DISPLAY_SCALE + ys) * width) + x) * DISPLAY_BYTES_PER_PIXEL) 
          * DISPLAY_SCALE;
        //  Scale the pixels horizontally to fill the canvas
        for (var xs = 0; xs < DISPLAY_SCALE; xs++) {
          const dest2 = dest + xs * DISPLAY_BYTES_PER_PIXEL;
          //  Copy 4 bytes: RGBA
          for (var b = 0; b < DISPLAY_BYTES_PER_PIXEL; b++) {
            data[dest2 + b] = Module.HEAPU8[addr + src + b];
          }
        }
      }
    }
  }
```

Note that JavaScript is allowed to read and write to WebAssembly Memory (treating it like a JavaScript array of bytes in `Module.HEAPU8[]`). But WebAssembly can't access any JavaScript Memory.

That's why we designed the Display Buffer Functions to manipulate WebAssembly Memory.

## Paint the HTML Canvas

Finally we update the HTML Canvas...

```javascript
  //  Paint the canvas
  ctx.putImageData(imageData, 0, 0);
}
```

# Install emscripten on Ubuntu x64

See the GitHub Actions Workflow...

[`.github/workflows/ccpp.yml`](.github/workflows/ccpp.yml)

Look for the steps...

1.   "Install emscripten"

1.   "Install wabt"

Change `/tmp` to a permanent path like `~`

Then add emscripten and wabt to the PATH...

```bash
# Add emscripten and wabt to the PATH
source ~/emsdk/emsdk_env.sh
export PATH=$PATH:~/wabt/build
```

# Install emscripten on Arch Linux / Manjaro Arm64

Works on Pinebook Pro with Manjaro...

```bash
sudo pacman -S emscripten
sudo pacman -S wabt
source /etc/profile.d/emscripten.sh
emcc --version
# Shows emscripten version 1.39.20
wasm-as --version
# Shows binaryen version 95
```

## For emscripten version 1.40.x and newer

emscripten and binaryen will probably work, skip the rest of this section.

## For emscripten version 1.39.x and binaryen version 95 only

This will fail during the build, because emscripten 1.39 needs binaryen version 93, not 95.

We could install binaryen version 93... But emcc will fail with an error "stackSave already exists". That's because binaryen 93 generates the "stackSave" that conflicts with emscripten 1.39.20. [More details here](https://github.com/emscripten-core/emscripten/pull/11166)

To fix this, we install binaryen version 94, __but rename it as version 93__...

```bash
# Download binaryen 94
git clone --branch version_94 https://github.com/WebAssembly/binaryen
cd binaryen
nano CMakeLists.txt 
```

Change
```
   project(binaryen LANGUAGES C CXX VERSION 94)
```
To
```
   project(binaryen LANGUAGES C CXX VERSION 93)
```

Then build and install binaryen...

```bash
cmake .
make -j 5
sudo cp bin/* /usr/bin
sudo cp lib/* /usr/lib
wasm-as --version
# Shows binaryen "version 93 (version_94)"
```

binaryen is now version 93, which is correct. Proceed to build the app...

```bash
cd lvgl-wasm
rm -rf ~/.emscripten_cache
make clean
make -j 5
```

The app build should complete successfully.

## emcc Error: Unexpected binaryen version

If we see this error...

```
   emcc: error: unexpected binaryen version: 95 (expected 93) [-Wversion-check] [-Werror]
   FAIL: Compilation failed!: ['/usr/lib/emscripten/emcc', '-D_GNU_SOURCE', '-o', '/tmp/tmpbe4ik5na.js', '/tmp/tmpzu5jusdg.c', '-O0', '--js-opts', '0', '--memory-init-file', '0', '-Werror', '-Wno-format', '-s', 'BOOTSTRAPPING_STRUCT_INFO=1', '-s', 'WARN_ON_UNDEFINED_SYMBOLS=0', '-s', 'STRICT=1', '-s', 'SINGLE_FILE=1']
```

## emcc Error: stackSave already exists

Then we need to install the right version of binaryen (see above)

If we see this error...

```
   Fatal: Module::addExport: stackSave already exists
   emcc: error: '/usr/bin/wasm-emscripten-finalize --detect-features --global-base=1024 --check-stack-overflow /tmp/emscripten_temp_84xtyzya/tmpzet09r88.wasm -o /tmp/emscripten_temp_84xtyzya/tmpzet09r88.wasm.o.wasm' failed (1)
   FAIL: Compilation failed!: ['/usr/lib/emscripten/emcc', '-D_GNU_SOURCE', '-o', '/tmp/tmpzet09r88.js', '/tmp/tmpxk8zxvza.c', '-O0', '--js-opts', '0', '--memory-init-file', '0', '-Werror', '-Wno-format', '-s', 'BOOTSTRAPPING_STRUCT_INFO=1', '-s', 'WARN_ON_UNDEFINED_SYMBOLS=0', '-s', 'STRICT=1', '-s', 'SINGLE_FILE=1']
```

That means binaryen 93 generates the "stackSave" that conflicts with emscripten 1.39.20. [More details here](https://github.com/emscripten-core/emscripten/pull/11166)

We need to install branch version_94 of binaryen, change version in CMakeLists.txt to version 93 (see above)

# Install emscripten on macOS (Doesn't Work)

Enter these commands...
```bash
brew install emscripten
brew install binaryen
# Upgrade llvm to 10.0.0
brew install llvm
brew upgrade llvm
nano /usr/local/Cellar/emscripten/1.40.1/libexec/.emscripten
```

Change BINARYEN_ROOT and LLVM_ROOT to 
```python
BINARYEN_ROOT = os.path.expanduser(os.getenv('BINARYEN', '/usr/local')) # directory
LLVM_ROOT = os.path.expanduser(os.getenv('LLVM', '/usr/local/opt/llvm/bin')) # directory
```

Fails with error:
```
   emcc: warning: LLVM version appears incorrect (seeing "10.0", expected "12.0") [-Wversion-check]
   shared:INFO: (Emscripten: Running sanity checks)
   clang-10: error: unknown argument: '-fignore-exceptions'
   emcc: error: '/usr/local/opt/llvm/bin/clang -target wasm32-unknown-emscripten -D__EMSCRIPTEN_major__=1 -D__EMSCRIPTEN_minor__=40 -D__EMSCRIPTEN_tiny__=1 -D_LIBCPP_ABI_VERSION=2 -Dunix -D__unix -D__unix__ -Werror=implicit-function-declaration -Xclang -nostdsysteminc -Xclang -isystem/usr/local/Cellar/emscripten/1.40.1/libexec/system/include/compat -Xclang -isystem/usr/local/Cellar/emscripten/1.40.1/libexec/system/include -Xclang -isystem/usr/local/Cellar/emscripten/1.40.1/libexec/system/include/libc -Xclang -isystem/usr/local/Cellar/emscripten/1.40.1/libexec/system/lib/libc/musl/arch/emscripten -Xclang -isystem/usr/local/Cellar/emscripten/1.40.1/libexec/system/local/include -Xclang -isystem/usr/local/Cellar/emscripten/1.40.1/libexec/system/include/SSE -Xclang -isystem/usr/local/Cellar/emscripten/1.40.1/libexec/system/lib/compiler-rt/include -Xclang -isystem/usr/local/Cellar/emscripten/1.40.1/libexec/system/lib/libunwind/include -Xclang -isystem/usr/local/Cellar/emscripten/1.40.1/libexec/cache/wasm/include -DEMSCRIPTEN -fignore-exceptions -Isrc/lv_core -D LV_USE_DEMO_WIDGETS ././src/lv_core/lv_group.c -Xclang -isystem/usr/local/Cellar/emscripten/1.40.1/libexec/system/include/SDL -c -o /var/folders/gp/jb0b68fn3b187mgyyrjml3km0000gn/T/emscripten_temp_caxv1fls/lv_group_0.o -mllvm -combiner-global-alias-analysis=false -mllvm -enable-emscripten-sjlj -mllvm -disable-lsr' failed (1)
```

# WebAssembly Stack Trace for PineTime Watch Face

TODO

Here is a sample WebAssembly Stack Trace that appears in the web browser. It happens when we don't initialise the LVGL Style `LabelBigStyle` used by [`Clock.cpp`](clock/Clock.cpp)

```
lvgl.js:1839 Fetch finished loading: GET "http://127.0.0.1:8887/lvgl.wasm".
instantiateAsync @ lvgl.js:1839
createWasm @ lvgl.js:1866
(anonymous) @ lvgl.js:2113
lvgl2.html:1237 In JavaScript: render_canvas()
lvgl2.html:1237 In C: Init display...
lvgl2.html:1237 Init display...
​ Uncaught RuntimeError: memory access out of bounds
    at _lv_style_get_int (http://127.0.0.1:8887/lvgl.wasm:wasm-function[229]:0x21bfb)
    at _lv_style_list_get_int (http://127.0.0.1:8887/lvgl.wasm:wasm-function[234]:0x22bf7)
    at _lv_obj_get_style_int (http://127.0.0.1:8887/lvgl.wasm:wasm-function[87]:0xe524)
    at lv_obj_get_style_shadow_width (http://127.0.0.1:8887/lvgl.wasm:wasm-function[162]:0x17d1b)
    at lv_obj_get_draw_rect_ext_pad_size (http://127.0.0.1:8887/lvgl.wasm:wasm-function[43]:0x70ae)
    at lv_obj_signal (http://127.0.0.1:8887/lvgl.wasm:wasm-function[33]:0x55e6)
    at lv_label_signal (http://127.0.0.1:8887/lvgl.wasm:wasm-function[261]:0x27804)
    at lv_obj_refresh_ext_draw_pad (http://127.0.0.1:8887/lvgl.wasm:wasm-function[45]:0x886f)
    at lv_obj_signal (http://127.0.0.1:8887/lvgl.wasm:wasm-function[33]:0x5793)
    at lv_label_signal (http://127.0.0.1:8887/lvgl.wasm:wasm-function[261]:0x27804)
_lv_style_get_int @ ​
_lv_style_list_get_int @ ​
_lv_obj_get_style_int @ ​
lv_obj_get_style_shadow_width @ ​
lv_obj_get_draw_rect_ext_pad_size @ ​
lv_obj_signal @ ​
lv_label_signal @ ​
lv_obj_refresh_ext_draw_pad @ ​
lv_obj_signal @ ​
lv_label_signal @ ​
lv_obj_refresh_style @ ​
lv_obj_add_style @ ​
Pinetime::Applications::Screens::Clock::Clock(DisplayApp*, Pinetime::Controllers::DateTime&, Pinetime::Controllers::Battery&, Pinetime::Controllers::Ble&) @ ​
create_clock @ ​
(anonymous) @ lvgl.js:1734
render_canvas @ lvgl2.html:1311
Module.onRuntimeInitialized @ lvgl2.html:1354
doRun @ lvgl.js:2496
(anonymous) @ lvgl.js:2509
setTimeout (async)
run @ lvgl.js:2505
runCaller @ lvgl.js:2411
removeRunDependency @ lvgl.js:1632
receiveInstance @ lvgl.js:1799
receiveInstantiatedSource @ lvgl.js:1816
Promise.then (async)
(anonymous) @ lvgl.js:1841
Promise.then (async)
instantiateAsync @ lvgl.js:1839
createWasm @ lvgl.js:1866
(anonymous) @ lvgl.js:2113
```

# Migrating LVGL Version 6 to 7

PineTime runs on LVGL version 6 while our WebAssembly port runs on LVGL version 7. And programs built for LVGL version 6 will not compile with LVGL version 7.

Here's how we migrated our code from LVGL Version 6...

- [Original PineTime Clock.cpp](https://github.com/JF002/Pinetime/blob/master/src/DisplayApp/Screens/Clock.cpp)

- [Original PineTime LittleVgl.cpp](https://github.com/JF002/Pinetime/blob/master/src/DisplayApp/LittleVgl.cpp)

To LVGL Version 7...

- [Converted WebAssembly Clock.cpp](clock/Clock.cpp)

- [Converted WebAssembly LittleVgl.cpp](clock/LittleVgl.cpp)

Compare the original and converted files...

- [Clock.cpp: LVGL Version 6 vs Version 7](https://github.com/AppKaki/lvgl-wasm/compare/clock_before...master#diff-9a3204013cda108f0edc5647e908ea82)

  Click `Files Changed`, then `Changed Files` and look for `Clock/Clock.cpp`

- [LittleVgl.cpp: LVGL Version 6 vs Version 7](https://github.com/AppKaki/lvgl-wasm/compare/AppKaki:littlevgl_before...master#diff-c2a76b9cd8a6d2fd824f1441a1e2ed34)

  Click `Files Changed`, then `Changed Files` and look for `Clock/LittleVgl.cpp`

## Migrating LVGL lv_label_set_style

Code that uses `lv_label_set_style`...

```c++
lv_label_set_style(label_time, LV_LABEL_STYLE_MAIN, LabelBigStyle);
```

Should be changed to `lv_obj_reset_style_list` and `lv_obj_add_style`...

```c++
//  Remove the styles coming from the theme
lv_obj_reset_style_list(label_time, LV_LABEL_PART_MAIN);
//  Then add style
lv_obj_add_style(label_time, LV_LABEL_PART_MAIN, LabelBigStyle);
```

Or define a macro like so...

```c++
/// Change LVGL v6 lv_label_set_style() to LVGL v7 lv_obj_reset_style_list() and lv_obj_add_style()
#define lv_label_set_style(label, style_type, style) \
{ \
    lv_obj_reset_style_list(label, LV_LABEL_PART_MAIN); \
    lv_obj_add_style(label, LV_LABEL_PART_MAIN, style); \
}
lv_label_set_style(label_time, LV_LABEL_STYLE_MAIN, LabelBigStyle);
```

`lv_obj_set_style()` may be defined like so...

```c
/// Change LVGL v6 lv_obj_set_style() to LVGL v7 lv_obj_reset_style_list() and lv_obj_add_style()
#undef lv_obj_set_style
void lv_obj_set_style(lv_obj_t *obj, const lv_style_t *style) {
    lv_obj_reset_style_list(obj, LV_LABEL_PART_MAIN);  //  TODO: Handle non-label styles
    lv_obj_add_style(obj, LV_LABEL_PART_MAIN, style);  //  TODO: Handle non-label styles
}
```

## Migrating LVGL lv_style_plain

`lv_style_plain` has been removed in LVGL 7. Code like this...

```c++
lv_style_copy(&def, &lv_style_plain);
```

Should be changed to...

```c++
lv_style_init(&def);
```

## Migrating LVGL Themes

In LVL 6, setting the default font for a Theme used to be easy...

```c++
lv_style_init(&def);
lv_style_set_text_font(&def, LV_STATE_DEFAULT, &jetbrains_mono_bold_20);
...
lv_theme_set_current(&theme);
```

But in LVL 7, we need to use [Theme Callback Functions](https://docs.lvgl.io/latest/en/html/overview/style.html?highlight=theme#themes) to apply the style.

A simpler solution is to set the default font in [`lv_conf.h`](lv_conf.h)...

```c++
#define LV_FONT_CUSTOM_DECLARE LV_FONT_DECLARE(jetbrains_mono_bold_20)
#define LV_THEME_DEFAULT_FONT_SMALL         &jetbrains_mono_bold_20
#define LV_THEME_DEFAULT_FONT_NORMAL        &jetbrains_mono_bold_20
#define LV_THEME_DEFAULT_FONT_SUBTITLE      &jetbrains_mono_bold_20
#define LV_THEME_DEFAULT_FONT_TITLE         &jetbrains_mono_bold_20
```

## Migrating LVGL Styles

Change LVL 6 Style...

```c++
lv_style_copy(&bg, &lv_style_plain);
bg.body.main_color = LV_COLOR_BLACK;
bg.body.grad_color = LV_COLOR_BLACK;
bg.text.color      = LV_COLOR_WHITE;
bg.text.font       = font;
bg.image.color     = LV_COLOR_WHITE;
```

To LVL 7 Style...

```c++
lv_style_init(&bg);
lv_style_set_bg_color(&bg, LV_STATE_DEFAULT, LV_COLOR_BLACK);
lv_style_set_bg_grad_color(&bg, LV_STATE_DEFAULT, LV_COLOR_BLACK);
lv_style_set_text_color(&bg, LV_STATE_DEFAULT, LV_COLOR_WHITE);
lv_style_set_text_font(&bg, LV_STATE_DEFAULT, font);
lv_style_set_image_recolor(&bg, LV_STATE_DEFAULT, LV_COLOR_WHITE);
```

Change LVL 6 Style...

```c++
lv_style_copy(&panel, &bg);
panel.body.main_color     = lv_color_hsv_to_rgb(hue, 11, 18);
panel.body.grad_color     = lv_color_hsv_to_rgb(hue, 11, 18);
panel.body.radius         = LV_DPI / 20;
panel.body.border.color   = lv_color_hsv_to_rgb(hue, 10, 25);
panel.body.border.width   = 1;
panel.body.border.opa     = LV_OPA_COVER;
panel.body.padding.left   = LV_DPI / 10;
panel.body.padding.right  = LV_DPI / 10;
panel.body.padding.top    = LV_DPI / 10;
panel.body.padding.bottom = LV_DPI / 10;
panel.line.color          = lv_color_hsv_to_rgb(hue, 20, 40);
panel.line.width          = 1;
```

To LVL 7 Style...

```c++
lv_style_copy(&panel, &bg);
lv_style_set_bg_color(&panel, LV_STATE_DEFAULT,       lv_color_hsv_to_rgb(hue, 11, 18)); 
lv_style_set_bg_grad_color(&panel, LV_STATE_DEFAULT,  lv_color_hsv_to_rgb(hue, 11, 18)); 
lv_style_set_radius(&panel, LV_STATE_DEFAULT,         LV_DPI / 20); 
lv_style_set_border_color(&panel, LV_STATE_DEFAULT,   lv_color_hsv_to_rgb(hue, 10, 25)); 
lv_style_set_border_width(&panel, LV_STATE_DEFAULT,   1); 
lv_style_set_border_opa(&panel, LV_STATE_DEFAULT,     LV_OPA_COVER); 
lv_style_set_pad_left(&panel, LV_STATE_DEFAULT,       LV_DPI / 10); 
lv_style_set_pad_right(&panel, LV_STATE_DEFAULT,      LV_DPI / 10); 
lv_style_set_pad_top(&panel, LV_STATE_DEFAULT,        LV_DPI / 10); 
lv_style_set_pad_bottom(&panel, LV_STATE_DEFAULT,     LV_DPI / 10); 
lv_style_set_line_color(&panel, LV_STATE_DEFAULT,     lv_color_hsv_to_rgb(hue, 20, 40)); 
lv_style_set_line_width(&panel, LV_STATE_DEFAULT,     1); 
```

For more examples of LVL Style migration, see...

[LittleVgl.cpp: LVGL Version 6 vs Version 7](https://github.com/AppKaki/lvgl-wasm/compare/AppKaki:littlevgl_before...master#diff-c2a76b9cd8a6d2fd824f1441a1e2ed34)

Click `Files Changed`, then `Changed Files` and look for `Clock/LittleVgl.cpp`


<h1 align="center"> LVGL - Light and Versatile Graphics Library</h1>

<p align="center">
<img src="https://lvgl.io/assets/images/img_1.png">
</p>

<p align="center">
LVGL provides everything you need to create embedded GUI with easy-to-use graphical elements, beautiful visual effects and low memory footprint. 
</p>

<h4 align="center">
<a href="https://lvgl.io">Website </a> &middot; 
<a href="https://lvgl.io/demos">Online demo</a> &middot; 
<a href="https://docs.lvgl.io/">Docs</a> &middot; 
<a href="https://forum.lvgl.io">Forum</a>
</h4>

---

## Features
* Powerful [building blocks](https://docs.lvgl.io/latest/en/html/widgets/index.html): buttons, charts, lists, sliders, images, etc.
* Advanced graphics: animations, anti-aliasing, opacity, smooth scrolling
* Use [various input devices](https://docs.lvgl.io/latest/en/html/overview/indev.html): touchscreen, mouse, keyboard, encoder, buttons, etc.
* Use [multiple displays](https://docs.lvgl.io/latest/en/html/overview/display.html): e.g. monochrome and color display
* Hardware independent to use with any microcontroller or display
* Scalable to operate with little memory (64 kB Flash, 10 kB RAM)
* Multi-language support with UTF-8 handling, Bidirectional and Arabic script support
* Fully customizable graphical elements via [CSS-like styles](https://docs.lvgl.io/latest/en/html/overview/style.html)
* OS, External memory and GPU are supported but not required
* Smooth rendering even with a [single frame buffer](https://docs.lvgl.io/latest/en/html/porting/display.html)
* Written in C for maximal compatibility (C++ compatible)
* Micropython Binding exposes [LVGL API in Micropython](https://blog.lvgl.io/2019-02-20/micropython-bindings)
* [Simulator](https://docs.lvgl.io/latest/en/html/get-started/pc-simulator.html) to develop on PC without embedded hardware
* [Examples](lv_examples) and tutorials for rapid development
* [Documentation](http://docs.lvgl.io/) and API references

## Requirements
Basically, every modern controller (which is able to drive a display) is suitable to run LVGL. The minimal requirements are:

<table>
  <tr>
    <td> <strong>Name</strong> </td>
    <td><strong>Minimal</strong></td>
    <td><strong>Recommended</strong></td>
  </tr>
  <tr>
    <td><strong>Architecture</strong></td>
    <td colspan="2">16, 32 or 64 bit microcontroller or processor</td>
  </tr>
  <tr>
    <td> <strong>Clock</strong></td>
    <td> &gt; 16 MHz </td>
    <td> &gt; 48 MHz</td>
  </tr>
  
  <tr>
    <td> <strong>Flash/ROM</strong></td>
    <td> &gt; 64 kB </td>
    <td> &gt; 180 kB</td>
  </tr>
  
  <tr>
    <td> <strong>Static RAM</strong></td>
    <td> &gt; 2 kB </td>
    <td> &gt; 4 kB</td>
  </tr>
  
  <tr>
    <td> <strong>Stack</strong></td>
    <td> &gt; 2 kB </td>
    <td> &gt; 8 kB</td>
  </tr>
  
  <tr>
    <td> <strong>Heap</strong></td>
    <td> &gt; 2 kB </td>
    <td> &gt; 8 kB</td>
  </tr>
  
  <tr>
    <td> <strong>Display buffer</strong></td>
    <td> &gt; 1 &times; <em>hor. res.</em> pixels </td>
    <td> &gt; 10 &times; <em>hor. res.</em> pixels </td>
  </tr>
  
  <tr>
    <td> <strong>Compiler</strong></td>
    <td colspan="2"> C99 or newer </td>
  </tr>
</table>

*Note that the memory usage might vary depending on the architecture, compiler and build options.*

Just to mention some platforms:
- STM32F1, STM32F3, STM32F4, STM32F7, STM32L4, STM32L5, STM32H7
- Microchip dsPIC33, PIC24, PIC32MX, PIC32MZ
- NXP: Kinetis, LPC, iMX, iMX RT
- [Linux frame buffer](https://blog.lvgl.io/2018-01-03/linux_fb) (/dev/fb)
- [Raspberry Pi](http://www.vk3erw.com/index.php/16-software/63-raspberry-pi-official-7-touchscreen-and-littlevgl)
- [Espressif ESP32](https://github.com/lvgl/lv_port_esp32)
- [Infineon Aurix](https://github.com/lvgl/lv_port_aurix)
- Nordic NRF52 Bluetooth modules
- Quectel modems

## Get started
This list shows the recommended way of learning the library:
1. Check the [Online demos](https://lvgl.io/demos) to see LVGL in action (3 minutes)
2. Read the [Introduction](https://docs.lvgl.io/latest/en/html/intro/index.html) page of the documentation (5 minutes)
3. Get familiar with the basics on the [Quick overview](https://docs.lvgl.io/latest/en/html/get-started/quick-overview.html) page (15 minutes)
4. Set up a [Simulator](https://docs.lvgl.io/latest/en/html/get-started/pc-simulator.html) (10 minutes)
5. Try out some [Examples](https://github.com/lvgl/lv_examples/)
6. Port LVGL to a board. See the [Porting](https://docs.lvgl.io/latest/en/html/porting/index.html) guide or check the ready to use [Projects](https://github.com/lvgl?q=lv_port_&type=&language=)
7. Read the [Overview](https://docs.lvgl.io/latest/en/html/overview/index.html) page to get a better understanding of the library (2-3 hours)
8. Check the documentation of the [Widgets](https://docs.lvgl.io/latest/en/html/widgets/index.html) to see their features and usage
9. If you have questions go to the [Forum](http://forum.lvgl.io/)
10. Read the [Contributing](https://docs.lvgl.io/latest/en/html/contributing/index.html) guide to see how you can help to improve LVGL (15 minutes) 

## Examples 

For more examples see the [lv_examples](https://github.com/lvgl/lv_examples) repository.

### Button with label
```c
lv_obj_t * btn = lv_btn_create(lv_scr_act(), NULL);     /*Add a button the current screen*/
lv_obj_set_pos(btn, 10, 10);                            /*Set its position*/
lv_obj_set_size(btn, 100, 50);                          /*Set its size*/
lv_obj_set_event_cb(btn, btn_event_cb);                 /*Assign a callback to the button*/

lv_obj_t * label = lv_label_create(btn, NULL);          /*Add a label to the button*/
lv_label_set_text(label, "Button");                     /*Set the labels text*/

...

void btn_event_cb(lv_obj_t * btn, lv_event_t event)
{
    if(event == LV_EVENT_CLICKED) {
        printf("Clicked\n");
    }
}
```
![LVGL button with label example](https://raw.githubusercontent.com/lvgl/docs/latest/misc/simple_button_example.gif)

### LVGL from Micropython
Learn more about [Micropython](https://docs.lvgl.io/latest/en/html/get-started/micropython.html).
```python
# Create a Button and a Label
scr = lv.obj()
btn = lv.btn(scr)
btn.align(lv.scr_act(), lv.ALIGN.CENTER, 0, 0)
label = lv.label(btn)
label.set_text("Button")

# Load the screen
lv.scr_load(scr)
```

## Contributing
LVGL is an open project and contribution is very welcome. There are many ways to contribute from simply speaking about your project, through writing examples, improving the documentation, fixing bugs to hosing your own project under in LVGL.

For a detailed description of contribution opportunities visit the [Contributing](https://docs.lvgl.io/latest/en/html/contributing/index.html) section of the documentation.
