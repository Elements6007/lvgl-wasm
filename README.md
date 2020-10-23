<h1 align="center"> WebAssembly Simulator for Rust on RIOT with LVGL  </h1>

![WebAssembly Simulator for Rust on RIOT](https://lupyuen.github.io/images/rust-on-riot-simulator.png)

__Note: This is the `rust` branch that supports Watch Faces built with Rust__

__Simulate Rust on RIOT Watch Face__ in Web Browser (with WebAssembly), for easier development of custom watch faces

- [Online Rust on RIOT Demo](https://appkaki.github.io/lvgl-wasm/rust.html)

- [Watch Face Source Code for Rust on RIOT](rust/app/src/watch_face.rs)

- Presentation Slides: [_Safer, Simpler Embedded Programs with Rust on RIOT_](https://docs.google.com/presentation/d/1IgCsWJ5TYpPaHXZlaETlM2rYQrBmOpN2WeFsNjmYO_k/edit?usp=sharing)

- Presentation Video: [_Safer, Simpler Embedded Programs with Rust on RIOT_](https://youtu.be/rTxeXnlH-mM)

Read the articles...

- ["Preview PineTime Watch Faces in your Web Browser with WebAssembly"](https://lupyuen.github.io/pinetime-rust-mynewt/articles/simulator)

- ["Porting PineTime Watch Face from C to Rust On RIOT with LVGL"](https://lupyuen.github.io/pinetime-rust-riot/articles/watch_face)

# Features

1. __Compiles actual Rust On RIOT Watch Face__ from Rust to WebAssembly without any code changes

1. __Uses GitHub Actions Workflow__ to build any fork of Rust On RIOT into WebAssembly

1. __Renders LVGL to HTML Canvas__ directly via WebAssembly, without using SDL2. See [`lvgl.html`](docs/lvgl.html#L1296-L1357)

1. __Supports RGB565 Framebuffer Format__ used by PineTime Display Controller, so that bitmaps will be rendered correctly. [Custom Bitmap Demo](https://appkaki.github.io/lvgl-wasm/lvgl2.html) / [Source Code](clock/Clock2.cpp)

1. __Shows current date and time__

# Upcoming Features

1. __Support Custom Fonts and Symbols__ for LVGL, by migrating LVGL v6 styles (used by pinetime-rust-riot) to v7 (used by lvgl-wasm)

1. __Accept Touch Input__ for LVGL

1. Allow Watch Faces to be __built online in Rust with online preview__

# References

- ["Porting PineTime Watch Face from C to Rust On RIOT with LVGL"](https://lupyuen.github.io/pinetime-rust-riot/articles/watch_face)

- Presentation Slides: ["Safer, Simpler Embedded Programs with Rust on RIOT"](https://docs.google.com/presentation/d/1IgCsWJ5TYpPaHXZlaETlM2rYQrBmOpN2WeFsNjmYO_k/edit?usp=sharing)

- Presentation Video: ["Safer, Simpler Embedded Programs with Rust on RIOT"](https://youtu.be/rTxeXnlH-mM)

- ["Preview PineTime Watch Faces in your Web Browser with WebAssembly"](https://lupyuen.github.io/pinetime-rust-mynewt/articles/simulator)

# How To Build The Simulator

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
