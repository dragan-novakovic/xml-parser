mod models;
use quick_xml::de;
use serde_json;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_xml(xml: &str, project_type: i32) -> String {
    match project_type {
        1 => {
            let model = de::from_str::<models::uwp::Project>(xml).unwrap();
            return serde_json::to_string(&model).unwrap();
        }
        2 => {
            let model = de::from_str::<models::maui::Project>(xml).unwrap();
            return serde_json::to_string(&model).unwrap();
        }
        _ => return "Unknown Project type".to_string(),
    };
}
