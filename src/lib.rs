use crate::input::parse_csv_string;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

mod input;
mod output;
mod tests;

#[wasm_bindgen]
pub fn parse_string(data: String) -> JsValue {
    let input_slice = data.as_str();
    let json_encoded = match parse_csv_string(input_slice) {
        Ok(v) => v,
        Err(e) => return JsValue::null(),
    };
    let slice = json_encoded.as_str();
    JsValue::from_str(slice)
}
