/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
//!  Main Rust Application for PineTime with Apache Mynewt OS
#![no_std]                              //  Don't link with standard Rust library, which is not compatible with embedded systems
#![feature(trace_macros)]               //  Allow macro tracing: `trace_macros!(true)`
#![feature(concat_idents)]              //  Allow `concat_idents!()` macro used in `coap!()` macro
#![feature(proc_macro_hygiene)]         //  Allow Procedural Macros like `run!()`
#![feature(specialization)]             //  Allow Specialised Traits for druid UI library
#![feature(exclusive_range_pattern)]    //  Allow ranges like `0..128` in `match` statements

//  Declare the libraries that contain macros
extern crate cortex_m;                  //  Declare the external library `cortex_m`
extern crate lvgl;                      //  Declare the LittlevGL (LVGL) library
extern crate macros as lvgl_macros;     //  Declare the LVGL Procedural Macros library

//  Declare the modules in our application
mod watch_face;             //  Declare `watch_face.rs` as Rust module `watch_face` for Watch Face

//  Declare the system modules
use core::panic::PanicInfo; //  Import `PanicInfo` type which is used by `panic()` below
use cortex_m::asm::bkpt;    //  Import cortex_m assembly function to inject breakpoint
use lvgl::console;          //  Import Semihosting Console functions

///  Main program, currently not used. TODO: Call at startup.
#[no_mangle]                 //  Don't mangle the function name
extern "C" fn rust_main() {  //  Declare extern "C" because it will be called by RIOT OS firmware
}

///  This function is called on panic, like an assertion failure. We display the filename and line number and pause in the debugger. From https://os.phil-opp.com/freestanding-rust-binary/
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    //  Display the filename and line number to the Semihosting Console.
    console::print("panic ");
    if let Some(location) = info.location() {
        let file = location.file();
        let line = location.line();
        console::print("at ");       console::buffer(&file);
        console::print(" line ");    console::printint(line as i32);
        console::print("\n");        console::flush();
    } else {
        console::print("no loc\n");  console::flush();
    }
    //  Pause in the debugger.
    bkpt();
    //  Display the payload.
    console::print(info.payload().downcast_ref::<&str>().unwrap());
    console::print("\n");  console::flush();
    //  Loop forever so that device won't restart.
    loop {}
}