mod json_model;

use serde_json::to_string_pretty;

impl crate::model::KeyboardDesc {
    pub fn to_json(&self) -> String {
        to_string_pretty(&json_model::Document::from_model(&self)).unwrap()
    }
}