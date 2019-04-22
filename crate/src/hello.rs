use wasm_bindgen::prelude::*;
use set_panic_hook;

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn hello(name: &str) -> Result<String, JsValue> {
    // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
    // it will do nothing.
    set_panic_hook();
    
    if name == "" { return Err(JsValue::from("name required!")) }

    let greeting = format!("Hello {}!", name);

    Ok(greeting)
}

