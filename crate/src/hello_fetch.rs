// use serde::{Deserialize, Serialize};
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use wasm_bindgen_futures::JsFuture;
// use web_sys::{Request, RequestInit, RequestMode, Response};
// use set_panic_hook;

// // Called by our JS entry point to run the example
// #[wasm_bindgen]
// pub async fn hello_fetch(name: &str) -> Result<JsValue, JsValue> {
//     set_panic_hook();
    
//     let mut opts = RequestInit::new();
//     opts.method("GET");
//     opts.mode(RequestMode::Cors);

//     let request = Request::new_with_str_and_init(
//         "https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master",
//         &opts,
//     )?;

//     request
//         .headers()
//         .set("Accept", "application/vnd.github.v3+json")?;

//     let window = web_sys::window().unwrap();
//     let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

//     // `resp_value` is a `Response` object.
//     assert!(resp_value.is_instance_of::<Response>());
//     let resp: Response = resp_value.dyn_into().unwrap();

//     // Convert this other `Promise` into a rust `Future`.
//     let json = JsFuture::from(resp.json()?).await?;

//     // Use serde to parse the JSON into a struct.
//     let branch_info: Branch = json.into_serde().unwrap();

//     // Send the `Branch` struct back to JS as an `Object`.
//     Ok(JsValue::from_serde(&branch_info).unwrap())
// }

