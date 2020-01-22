use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;
use crate::set_panic_hook;


#[wasm_bindgen]
pub async fn async_hello() -> Result<JsValue, JsValue> {
    set_panic_hook();
    
    let greeting = JsValue::from("hello!");

    let promise = Promise::resolve(&greeting.into());
    let result = JsFuture::from(promise).await?;
    Ok(result)
}
