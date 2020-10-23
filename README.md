<h1 align="center"> WebAssembly Simulator for Rust on Mynewt with LVGL  </h1>

![WebAssembly Simulator for Rust on Mynewt](https://lupyuen.github.io/images/rust-on-mynewt-simulator.png)

__Note: This is the `mynewt` branch that supports Watch Faces built with Rust on Mynewt__

__Simulate Rust on Mynewt Watch Face__ in Web Browser (with WebAssembly), for easier development of custom watch faces

- [Online Simulator Demo](https://lupyuen.github.io/barebones-watchface/lvgl.html)

- [Watch Face Source Code](https://github.com/lupyuen/barebones-watchface/blob/master/src/lib.rs)

- [Watch Face Crate on crates.io](https://crates.io/crates/barebones-watchface)

- [GitHub Actions Workflow for Simulator](https://github.com/lupyuen/barebones-watchface/blob/master/.github/workflows/simulator.yml)

# Features

1. __Compiles actual Rust On Mynewt Watch Face__ from Rust to WebAssembly without any code changes

1. __Uses GitHub Actions Workflow__ to build any fork of Rust On Mynewt into WebAssembly

1. __Renders LVGL to HTML Canvas__ directly via WebAssembly, without using SDL2. See [`lvgl.html`](docs/lvgl.html#L1296-L1357)

1. __Supports RGB565 Framebuffer Format__ used by PineTime Display Controller, so that bitmaps will be rendered correctly

1. __Shows current date and time__

# Upcoming Features

1. __Support Custom Fonts and Symbols__ for LVGL

1. __Accept Touch Input__ for LVGL

1. Allow Watch Faces to be __built online in Rust with online preview__

# References

- ["Create Your Own PineTime Watch Face in Rust... And Publish on crates.io"](https://lupyuen.github.io/pinetime-rust-mynewt/articles/watchface)

- ["Preview PineTime Watch Faces in your Web Browser with WebAssembly"](https://lupyuen.github.io/pinetime-rust-mynewt/articles/simulator)

# How To Build The Simulator

To build PineTime Watch Face Simulator on Linux, macOS or Windows (without WSL), follow these steps based on the GitHub Actions Workflow [`.github/workflows/simulator.yml`](.github/workflows/simulator.yml)...

1.  Install emscripten and wabt. See instructions below.

1.  Install Rust from `rustup.rs`

1.  Enter...

    ```bash
    git clone --branch mynewt  https://github.com/AppKaki/lvgl-wasm
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

    Here is the `sed` script that makes the change...

    ```bash
    cat $HOME/.cargo/registry/src/github.com-*/cty-0.1.5/src/lib.rs \
        | sed 's/target_arch = "arm"/target_arch = "arm", target_arch = "wasm32"/' \
        >/tmp/lib.rs
    cp /tmp/lib.rs $HOME/.cargo/registry/src/github.com-*/cty-0.1.5/src/lib.rs
    rm /tmp/lib.rs
    ```
        
1.  (Optional) Inject the Watch Face Crate into lvgl-wasm.

    By default lvgl-wasm builds the WebAssembly Simulator with the Barebones Watch Face. Skip this step to use the default Watch Face.
    
    Assuming that `my_watchface::MyWatchFace` is the Watch Face that will be built into the Simulator...

    Change `mynewt/wasm/Cargo.toml` from

    ```
    barebones-watchface = "x.x.x"
    ```

    to

    ```
    # If my-watchface is on crates.io...
    my-watchface = "x.x.x"

    # If my-watchface is on GitHub...
    my-watchface = { git = "https://github.com/..." }
    ```

    Change `mynewt/wasm/src/lib.rs` from

    ```
    use barebones_watchface::watchface::lvgl::mynewt::fill_zero;
    use barebones_watchface::watchface::{self, WatchFace};
    type WatchFaceType = barebones_watchface::BarebonesWatchFace;
    ```

    to

    ```
    use my_watchface::watchface::lvgl::mynewt::fill_zero;
    use my_watchface::watchface::{self, WatchFace};
    type WatchFaceType = my_watchface::MyWatchFace;
    ```

    For the `sed` script, see [`barebones-watchface/.github/workflows/simulator.yml`](https://github.com/lupyuen/barebones-watchface/blob/master/.github/workflows/simulator.yml#L170-L250)

1.  Build the LVGL WebAssembly app containing our Watch Face...

    ```bash
    cargo build
    source emsdk/emsdk_env.sh
    make -f mynewt/Makefile
    ```
    
    For Windows (without WSL): See the build instructions in [`mynewt/build.cmd`](mynewt/build.cmd)

1.  If we see this error...

    ```
    emscripten:ERROR: emscript: failure to parse metadata output from wasm-emscripten-finalize
    ```

    Revert to the older working version of emscripten...

    ```bash
    emsdk/emsdk install 2.0.6
    make -f mynewt/Makefile
    ```

1.  Copy the generated WebAssembly files to the `docs` folder (used by GitHub Pages)...

    ```bash
    cp wasm/lvgl.js wasm/lvgl.wasm docs
    ```
    
    For Windows (without WSL):

    ```cmd
    copy wasm/lvgl.js wasm/lvgl.wasm docs
    ```

    We don't need `lvgl.html` because `docs` already contains a version of `lvgl.html` with custom JavaScript.

1.  Start a Web Server for the `docs` folder, because WebAssembly doesn't work when opened from the filesystem.

    __For Linux:__ Use [`darkhttpd`](https://unix4lyfe.org/darkhttpd/)...

    ```bash
    darkhttpd docs
    ```

    __For macOS and Windows:__ Use the Chrome Extension [Web Server for Chrome](https://chrome.google.com/webstore/detail/web-server-for-chrome/ofhbbkphhbklhfoeikjpcbhemlocgigb?hl=en) and set the folder to `docs`

1.  Launch a Web Browser and open the URL shown by `darkhttpd` or Web Server for Chrome.

    Enter `lvgl.html` in the URL bar to view the PineTime Watch Face Simulator.

In case of problems, compare with the following...

- [GitHub Actions workflow](.github/workflows/simulator.yml)

- [GitHub Actions build log](https://github.com/AppKaki/lvgl-wasm/actions) (look for `mynewt` branch)


# Install emscripten on Ubuntu x64 and Windows WSL

To install emscripten on Ubuntu x64 and Windows WSL...

```bash
# Get the emsdk repo
git clone https://github.com/emscripten-core/emsdk.git

# Enter that directory
cd emsdk

# Download and install version 2.0.6 of the SDK tools. The latest version 2.0.7 fails to build lvgl-wasm.
./emsdk install 2.0.6
        
# Make version 2.0.6 active for the current user (writes .emscripten file)
./emsdk activate 2.0.6

# Activate PATH and other environment variables in the current terminal
source ./emsdk_env.sh

# Show version
emcc --version
emcc --version 
```

Alternatively, use the prebuilt emscripten binary for Ubuntu x64...

```bash
# Download the emscripten binary from lvgl-wasm
curl -L https://github.com/AppKaki/lvgl-wasm/releases/download/v1.0.0/emsdk.tgz -o emsdk.tgz

# Unzip to emsdk
tar xzf emsdk.tgz
rm emsdk.tgz

# Activate PATH and other environment variables in the current terminal
source emsdk/emsdk_env.sh

# Show version
emcc --version
emcc --version        
```

This is based on the GitHub Actions Workflow: [`.github/workflows/simulator.yml`](.github/workflows/simulator.yml)...

1.  Look for the steps "Install emscripten" and "Install wabt"

1.  Change `/tmp` to a permanent path like `~`

1.  Then add emscripten and wabt to the PATH...

    ```bash
    # Add emscripten and wabt to the PATH
    source ~/emsdk/emsdk_env.sh
    export PATH=$PATH:~/wabt/build
    ```

# Install emscripten on macOS

Enter these commands [according to the docs](https://emscripten.org/docs/getting_started/downloads.html#installation-instructions)...

```bash
# Get the emsdk repo
git clone https://github.com/emscripten-core/emsdk.git

# Enter that directory
cd emsdk

# Download and install version 2.0.6 of the SDK tools. The latest version 2.0.7 fails to build lvgl-wasm.
./emsdk install 2.0.6
        
# Make version 2.0.6 active for the current user (writes .emscripten file)
./emsdk activate 2.0.6

# Activate PATH and other environment variables in the current terminal
source ./emsdk_env.sh

# Show version
emcc --version
emcc --version 
```

If we see this error...

```
+ exec python ./emsdk.py install latest
Installing SDK 'sdk-releases-upstream-d7a29d82b320e471203b69d43aaf03b560eedc54-64bit'..
Installing tool 'node-12.18.1-64bit'..
Error: Downloading URL 'https://storage.googleapis.com/webassembly/emscripten-releases-builds/deps/node-v12.18.1-darwin-x64.tar.gz': <urlopen error [SSL: CERTIFICATE_VERIFY_FAILED] certificate verify failed (_ssl.c:777)>
Warning: Possibly SSL/TLS issue. Update or install Python SSL root certificates (2048-bit or greater) supplied in Python folder or https://pypi.org/project/certifi/ and try again.
Installation failed!
```

Try installing the latest Python 3 via `brew install`. Then edit the shell script [`emsdk/emsdk`](https://github.com/emscripten-core/emsdk/blob/master/emsdk) and set `EMSDK_PYTHON` to the path of the installed Python 3 executable...

```
# Insert this line before exec
EMSDK_PYTHON=/usr/local/Cellar/python@3.8/3.8.5/bin/python3

exec "$EMSDK_PYTHON" "$0.py" "$@"
```

# Install emscripten on Windows Without WSL

To install emscripten on plain old Windows without WSL...

```bash
# Get the emsdk repo
git clone https://github.com/emscripten-core/emsdk.git

# Enter that directory
cd emsdk

# Download and install version 2.0.6 of the SDK tools. The latest version 2.0.7 fails to build lvgl-wasm.
emsdk.bat install 2.0.6
        
# Make version 2.0.6 active for the current user (writes .emscripten file)
emsdk.bat activate 2.0.6

# Activate PATH and other environment variables in the current terminal
emsdk_env.bat

# Show version
emcc --version
emcc --version 
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

Then we need to install the right version of binaryen (see above)

## emcc Error: stackSave already exists

If we see this error...

```
   Fatal: Module::addExport: stackSave already exists
   emcc: error: '/usr/bin/wasm-emscripten-finalize --detect-features --global-base=1024 --check-stack-overflow /tmp/emscripten_temp_84xtyzya/tmpzet09r88.wasm -o /tmp/emscripten_temp_84xtyzya/tmpzet09r88.wasm.o.wasm' failed (1)
   FAIL: Compilation failed!: ['/usr/lib/emscripten/emcc', '-D_GNU_SOURCE', '-o', '/tmp/tmpzet09r88.js', '/tmp/tmpxk8zxvza.c', '-O0', '--js-opts', '0', '--memory-init-file', '0', '-Werror', '-Wno-format', '-s', 'BOOTSTRAPPING_STRUCT_INFO=1', '-s', 'WARN_ON_UNDEFINED_SYMBOLS=0', '-s', 'STRICT=1', '-s', 'SINGLE_FILE=1']
```

That means binaryen 93 generates the "stackSave" that conflicts with emscripten 1.39.20. [More details here](https://github.com/emscripten-core/emscripten/pull/11166)

We need to install branch version_94 of binaryen, change version in CMakeLists.txt to version 93 (see above)

# How It Works

The WebAssembly Simulator is compiled as multiple parts...

1. Watch Face Library in Rust, from [`mynewt/wasm`](mynewt/wasm)

   Compiled with `cargo` with target `wasm32-unknown-emscripten` into a WebAssembly Static Library: `target/wasm32-unknown-emscripten/debug/libwasm.a`
   
   Contains the selected Watch Face code and WebAssembly interface [`mynewt/wasm`](mynewt/wasm)
   
1. LVGL Library in C, from [`src`](src)

   Compiled from C to WebAssembly with [emscripten](https://developer.mozilla.org/en-US/docs/WebAssembly/C_to_wasm)

1. WebAssembly Interface in C, from [`wasm/lvgl.c`](wasm/lvgl.c)

   Compiled from C to WebAssembly with [emscripten](https://developer.mozilla.org/en-US/docs/WebAssembly/C_to_wasm)

The Makefile [`mynewt/Makefile`](mynewt/Makefile) links the above into WebAssembly like this...

```bash
emcc -o wasm/lvgl.html \
	-Wl,--start-group \
  target/wasm32-unknown-emscripten/debug/libwasm.a \
	(List of C and C++ object files from LVGL and Sandbox) \
	-Wl,--end-group \
	-g \
	-I src/lv_core \
	-D LV_USE_DEMO_WIDGETS \
	-s WASM=1 \
    -s "EXPORTED_FUNCTIONS=[ '_main', '_get_display_buffer', '_get_display_width', '_get_display_height', '_test_display', '_init_display', '_render_display', '_render_widgets', '_create_clock', '_refresh_clock', '_update_clock' ]"
```

The emscripten compiler `emcc` generates three files in folder `wasm`...

- `lvgl.wasm`: WebAssembly Executable Code, containing our Watch Face, LVGL and the Sandbox. [Sample File](docs/lvgl.wasm)

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

# Rust on Mynewt Sandbox

Our WebAssembly Simulator runs in a Web Browser based on WebAssembly (somewhat similar to Java Applets). [More about WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Concepts)

Our Watch Face Crate calls functions from two providers...

1. [LVGL Library (Version 7)](https://docs.lvgl.io/latest/en/html/index.html) for UI widgets

1. [PineTime Watch Face Framework](https://github.com/lupyuen/pinetime-watchface) for current date / time, Bluetooth status and power status

`lvgl-wasm` simulates the minimal set of functions needed for rendering Watch Faces.

Hence `lvgl-wasm` works like a __Sandbox__.  Here's how the Sandbox works...

## Exported Functions

The Sandbox exports the following WebAssembly functions from Rust to JavaScript...

### Clock Functions

These functions create the Watch Face, render the LVGL widgets on the Watch Face, and update the time...

-   `create_clock()`

    Create the Watch Face. Calls [`pinetime-watchface`](https://github.com/lupyuen/pinetime-watchface) to create an instance of the Watch Face from the Watch Face Crate.
    
    From [`mynewt/wasm/src/lib.rs`](mynewt/wasm/src/lib.rs)

-   `refresh_clock()`

    Redraw the Watch Face. Currently does nothing.
    
    From [`mynewt/wasm/src/lib.rs`](mynewt/wasm/src/lib.rs)

-   `update_clock(year, month, day, hour, minute, second)`

    Set the current date and time. Calls [`pinetime-watchface`](https://github.com/lupyuen/pinetime-watchface) to redraw the instance of the Watch Face from the Watch Face Crate.

    From [`mynewt/wasm/src/lib.rs`](mynewt/wasm/src/lib.rs)

    TODO: This code needs to be fixed to show the correct day of week
    
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
