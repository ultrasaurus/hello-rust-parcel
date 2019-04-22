use wasm_bindgen::prelude::*;
use set_panic_hook;

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn hello(name: &str) -> Result<String, JsValue> {
    set_panic_hook();
    
    if name == "" { return Err(JsValue::from("name required!")) }

    let greeting = format!("Hello {}!", name);

    Ok(greeting)
}

