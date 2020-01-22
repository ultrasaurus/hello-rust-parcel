extern crate js_sys;

use wasm_bindgen::prelude::*;
use crate::set_panic_hook;

#[wasm_bindgen]
pub fn hello_array(count: i32) -> Result<js_sys::Array, JsValue> {
    set_panic_hook();
    let result = js_sys::Array::new();

    for i in 0..count {
      let content = JsValue::from_str(&i.to_string());
      result.push(&content);
    }

    Ok(result)
}


