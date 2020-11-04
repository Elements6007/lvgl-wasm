/* To build:
rustup default nightly
rustup target add wasm32-unknown-emscripten
cargo build
*/
#![feature(libc)]  //  Allow C Standard Library, which will be mapped by emscripten to JavaScript

use barebones_watchface::watchface::lvgl::mynewt::fill_zero;
use barebones_watchface::watchface::{self, WatchFace};   //  Needed for calling WatchFace traits

mod script;

/// Declare the Watch Face Type
type WatchFaceType = barebones_watchface::BarebonesWatchFace;

/// Watch Face for the app
static mut WATCH_FACE: WatchFaceType = fill_zero!(WatchFaceType);

/// Create an instance of the clock
#[no_mangle]
pub extern fn create_clock() -> i32 {
    unsafe { puts(b"In Rust: Creating clock...\0".as_ptr()); }

    /*
    //  Create the watch face
    unsafe {  //  Unsafe because WATCH_FACE is a mutable static  
        WATCH_FACE = WatchFaceType::new()
            .expect("Create watch face fail");
    }
    */

    //  Run the script
    script::run_script().unwrap();

    //  Return OK, caller will render display
    0
}

/// Redraw the clock
#[no_mangle]
pub extern fn refresh_clock() -> i32 {
    unsafe { puts(b"In Rust: Refreshing clock...\0".as_ptr()); }

    //  Return OK, caller will render display
    0
}

/// Update the clock time. Use generic "int" type to prevent JavaScript-WebAssembly interoperability problems.
#[no_mangle]
pub extern fn update_clock(year: i32, month: i32, day: i32,
    hour: i32, minute: i32, second: i32) -> i32 {
    unsafe { puts(b"In Rust: Updating clock...\0".as_ptr()); }

    /*
    //  Compose the state
    let state = watchface::WatchFaceState {
        time:       watchface::WatchFaceTime {
            year:       year   as u16,
            month:      month  as u8,
            day:        day    as u8,
            hour:       hour   as u8,
            minute:     minute as u8,
            second:     second as u8,
            day_of_week: 1,  //  TODO
        },
        bluetooth:  watchface::BluetoothState::BLUETOOTH_STATE_CONNECTED,
        millivolts: 0,
        charging:   true,
        powered:    true,
    };

    //  Update the watch face
    unsafe {  //  Unsafe because WATCH_FACE is a mutable static
        WATCH_FACE.update(&state)
            .expect("Update watch face fail");
    }
    */
    
    //  Return OK, caller will render display
    0
}

extern "C" {
    /// Print to the JavaScript Console. From Standard C Library, mapped to JavaScript by emscripten.
    fn puts(fmt: *const u8) -> i32;
    //  fn printf(fmt: *const u8, ...) -> i32;
}

///////////////////////////////////////////////////////////////////////////////
//  Test Functions

#[no_mangle]
pub extern fn test_rust() -> i32 {
    unsafe { puts(b"In Rust: test_rust()\0".as_ptr()); }
    2205
}

#[no_mangle]
pub extern fn test_rust2() -> i32 {
    unsafe { puts(b"In Rust: test_rust2()\0".as_ptr()); }
    2306
}

#[no_mangle]
pub extern fn test_rust3() -> i32 {
    unsafe { puts(b"In Rust: test_rust3()\0".as_ptr()); }
    let i = unsafe { test_c() };
    i
}

#[no_mangle]
pub extern fn test_rust_set_buffer() -> i32 {
    unsafe { puts(b"In Rust: test_rust_set_buffer()\0".as_ptr()); }
    let i = unsafe { test_rust_buffer[0] };
    unsafe { test_rust_buffer[0] = 0x42; }  //  B
    i as i32
}

#[no_mangle]
pub extern fn test_rust_get_buffer() -> i32 {
    unsafe { puts(b"In Rust: test_rust_get_buffer()\0".as_ptr()); }
    unsafe { test_rust_buffer[0] as i32 }
}

extern "C" {
    static mut test_rust_buffer: [u8; 32];
    fn test_c() -> i32;
}
