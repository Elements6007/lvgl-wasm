/* To build:
rustup default nightly
rustup target add wasm32-unknown-emscripten
cargo build --target=wasm32-unknown-emscripten
*/
#![feature(libc)]

use core::ptr;
use app::watch_face;

static mut WIDGETS: watch_face::WatchFaceWidgets = watch_face::WatchFaceWidgets {
    screen:      ptr::null_mut(),
    time_label:  ptr::null_mut(),
    date_label:  ptr::null_mut(),
    ble_label:   ptr::null_mut(),
    power_label: ptr::null_mut(),
};

/// Create an instance of the clock
#[no_mangle]
pub extern fn create_clock() -> i32 {
    unsafe { puts(b"In Rust: Creating clock...\0".as_ptr()); }
    unsafe {
        WIDGETS.screen = get_screen();
        watch_face::create_widgets(&mut WIDGETS)
            .expect("create_widgets failed");
    }
    0
}

/// Redraw the clock
#[no_mangle]
pub extern fn refresh_clock() -> i32 {
    unsafe { puts(b"In Rust: Refreshing clock...\0".as_ptr()); }
    0
}

/// Update the clock time. Use generic "int" type to prevent JavaScript-WebAssembly interoperability problems.
#[no_mangle]
pub extern fn update_clock(year: i32, month: i32, day: i32,
    hour: i32, minute: i32, second: i32) -> i32 {
    unsafe { puts(b"In Rust: Updating clock...\0".as_ptr()); }
    let state = watch_face::WatchFaceState {
        ble_state:  watch_face::BleState::BLEMAN_BLE_STATE_CONNECTED ,
        time:       watch_face::controller_time_spec_t {
            year:       year as u16,
            month:      month as u8,
            dayofmonth: day as u8,
            hour:       hour as u8,
            minute:     minute as u8,
            second:     second as u8,
            fracs:      0,
        },
        millivolts: 0,
        charging:   true,
        powered:    true,
    };
    unsafe {
        watch_face::set_time_label(&mut WIDGETS, &state)
            .expect("set_time_label failed");
    }
    0
}

extern "C" {
    /// Get LVGL Screen. Defined in wasm/lvgl.c
    fn get_screen() -> *mut lvgl::core::obj::lv_obj_t;
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

/*
//  use std::ops::Add;
//  use wasm_bindgen::prelude::*;
//  use wasm_bindgen::Clamped;
//  use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    real: f64,
    imaginary: f64,
) -> Result<(), JsValue> {
    // The real workhorse of this algorithm, generating pixel data
    let c = Complex { real, imaginary };
    let mut data = get_julia_set(width, height, c);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn get_julia_set(width: u32, height: u32, c: Complex) -> Vec<u8> {
    let mut data = Vec::new();

    let param_i = 1.5;
    let param_r = 1.5;
    let scale = 0.005;

    for x in 0..width {
        for y in 0..height {
            let z = Complex {
                real: y as f64 * scale - param_r,
                imaginary: x as f64 * scale - param_i,
            };
            let iter_index = get_iter_index(z, c);
            data.push((iter_index / 4) as u8);
            data.push((iter_index / 2) as u8);
            data.push(iter_index as u8);
            data.push(255);
        }
    }

    data
}

fn get_iter_index(z: Complex, c: Complex) -> u32 {
    let mut iter_index: u32 = 0;
    let mut z = z;
    while iter_index < 900 {
        if z.norm() > 2.0 {
            break;
        }
        z = z.square() + c;
        iter_index += 1;
    }
    iter_index
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    fn square(self) -> Complex {
        let real = (self.real * self.real) - (self.imaginary * self.imaginary);
        let imaginary = 2.0 * self.real * self.imaginary;
        Complex { real, imaginary }
    }

    fn norm(&self) -> f64 {
        (self.real * self.real) + (self.imaginary * self.imaginary)
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}
*/