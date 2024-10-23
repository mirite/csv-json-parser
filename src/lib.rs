use crate::input::{parse_csv_string, parse_document};
use js_sys::{Array, Object};
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
        Err(e) => return JsValue::from_str(&e.to_string()),
    };
    let slice = json_encoded.as_str();
    JsValue::from_str(slice)
}

#[wasm_bindgen]
pub fn parse_to_object(data: String) -> JsValue {
    let input_slice = data.as_str();
    let parsed = match parse_document(input_slice) {
        Ok(v) => v,
        Err(e) => return JsValue::from_str(&e.to_string()),
    };

    JsValue::from(ParsedDocument {
        rows: parsed.1,
        keys: parsed.0,
    })
}

pub struct ParsedDocument {
    pub keys: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl From<ParsedDocument> for JsValue {
    fn from(parsed: ParsedDocument) -> JsValue {
        let column_count = parsed.keys.len();

        let rows_array = Array::new();
        for row in parsed.rows {
            let row_object = Object::new();
            for index in 0..column_count {
                js_sys::Reflect::set(
                    &row_object,
                    &JsValue::from_str(&parsed.keys[index]),
                    &JsValue::from_str(&row[index]),
                )
                .unwrap();
            }
            rows_array.push(&row_object);
        }

        JsValue::from(rows_array)
    }
}
