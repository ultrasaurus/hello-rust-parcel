// arbitrary data with Serde
// https://rustwasm.github.io/docs/wasm-bindgen/reference/arbitrary-data-with-serde.html
use wasm_bindgen::prelude::*;
use set_panic_hook;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct BookStoreData {
    pub h: HashMap<String, String>,
    pub name: String,
}

#[wasm_bindgen]
pub fn hello_hash(count: i32) -> Result<JsValue, JsValue> {
    set_panic_hook();
    // https://doc.rust-lang.org/std/collections/struct.HashMap.html
    let mut book_reviews = HashMap::new();

    // Review some books.
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    let data = BookStoreData {
        h: book_reviews,
        name: "My Book Store".to_string(),
    };

    let js_result: JsValue = JsValue::from_serde(&data).unwrap();

    Ok(js_result)    

}


