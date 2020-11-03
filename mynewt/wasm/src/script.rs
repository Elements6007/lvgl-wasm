//! Rhai Scripting for LVGL in WebAssembly (looks like Rust)
//! https://schungx.github.io/rhai/about/index.html
use rhai::{
    Engine,
    EvalAltResult,
    RegisterFn,
    //RegisterResultFn,
    packages::{
        BasicStringPackage,
        Package,
    },
};
use barebones_watchface::watchface::{self};   //  Needed for calling WatchFace traits

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

    //  Register an LVGL function
    engine.register_fn(
        "watchface_get_active_screen",  //  Name of Rhai function
        watchface::get_active_screen    //  LVGL function
    );

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