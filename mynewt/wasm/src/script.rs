//! Rhai Scripting for LVGL in WebAssembly (looks like Rust)
//! https://schungx.github.io/rhai/about/index.html
use core::ptr;
use rhai::{
    Dynamic,
    Engine,
    EvalAltResult,
    RegisterFn,
    RegisterResultFn,
    // packages::{
    //     BasicStringPackage,
    //     Package,
    // },
};
use barebones_watchface::{
    watchface::{
        self,
        lvgl::{
            self,
            core::obj,
            widgets::{
                label,
            },
            mynewt::{
                // self,
                Strn,
            }
        },
    },
};

/// Run a Rhai script that calls LVGL functions in WebAssembly
pub fn run_script() -> Result<(), Box<EvalAltResult>> {
    //  Create the script engine
    let mut engine = Engine::new();

    //  Add callbacks to support `print()` and `debug()` in script
    engine.on_print(|x| println!("print: {}", x));
    engine.on_debug(|x| println!("DEBUG: {}", x));

    //  Load the Basic String Package into the script engine
    //  let package = BasicStringPackage::new();
    //  engine.load_package(package.get());

    //  Register the LVGL functions
    engine.register_fn(
        "watchface_get_active_screen",  //  Name of Rhai function
        watchface::get_active_screen    //  LVGL function
    );
    engine.register_fn("ptr_null", ptr_null);  //  TODO: Rewrite as ptr::null
    engine.register_result_fn("label_create", label_create);  //  TODO: Rewrite as label::create
    engine.register_result_fn("label_set_text", label_set_text);  //  TODO: Rewrite as label::set_text
    //  engine.register_result_fn("obj_set_width", obj::set_width);
    //  engine.register_result_fn("obj_set_height", obj::set_height);

    //  Execute the Rhai script
    let result = engine.eval::<i64>(r#"
        //  Here is the Rhai script (looks like Rust), which can be modified at runtime...
        //  Call an LVGL function
        let screen = watchface_get_active_screen();

        //  Print a message
        print("Hello from Rhai script in WebAssembly!");

        //  Return the result
        40 + 2
    "#)?;
    println!("Answer: {}", result);  // prints 42
    Ok(())
}

fn ptr_null() -> *const obj::lv_obj_t {
    ptr::null()
}

//  LVGL Functions mapped to Rhai calling convention
//  TODO: Generate automatically with the `safe_wrap` proc macro

fn label_create(
    par: *mut obj::lv_obj_t, 
    copy: *const obj::lv_obj_t
) -> Result<Dynamic, Box<EvalAltResult>> {
    let result = label::create(par, copy)
        .expect("label_create fail");
    //  TODO: Ok(result.into())
    Ok(().into())
}

fn label_set_text(lbl: lvgl::Ptr, _s: &str) -> Result<Dynamic, Box<EvalAltResult>> {    
    let result = label::set_text(lbl, macros::strn!("TODO"))
        .expect("label_set_text fail");
    Ok(result.into())
}

/*
let lbl = label::create(screen, ptr::null()) ? ;  //  `?` will terminate the function in case of error
label::set_text(     lbl, strn!("00:00")) ? ;     //  strn creates a null-terminated string
obj::set_width(      lbl, 240) ? ;
obj::set_height(     lbl, 200) ? ;
*/