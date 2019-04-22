extern crate js_sys;

use wasm_bindgen::prelude::*;
use set_panic_hook;

#[wasm_bindgen]
pub fn hello_array(count: i32) -> Result<js_sys::Array, JsValue> {
    set_panic_hook();
    let result = js_sys::Array::new();

    let content = JsValue::from_str(&count.to_string());
    for _i in 0..count {
        result.push(&content);
    }

    Ok(result)
}


