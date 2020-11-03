use rhai::{
    Engine,
    EvalAltResult,
    RegisterFn,
    //RegisterResultFn,
};
use barebones_watchface::watchface::{self};   //  Needed for calling WatchFace traits

pub fn run_script() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.register_fn("watchface_get_active_screen", watchface::get_active_screen);
    //let result = engine.eval::<i64>("watchface_get_active_screen()")?;

    let result = engine.eval::<i64>("40 + 2")?;
    println!("Answer: {}", result);             // prints 42
    Ok(())
}