mod models;
use quick_xml::{de, DeError};
use serde_json;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_xml(xml: &str, project_type: i32) -> String {
    let deserialized_model = match project_type {
        1 => de::from_str::<models::uwp::Project>(xml),
        _ => Err(DeError::Custom("Unknown Project type".to_string())),
    };

    let result = serde_json::to_string(&deserialized_model.unwrap()).unwrap();
    result
}
