use rhai::{Engine, EvalAltResult};

pub fn run_script() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    let result = engine.eval::<i64>("40 + 2")?;

    println!("Answer: {}", result);             // prints 42

    Ok(())
}