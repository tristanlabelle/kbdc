
use std::collections::BTreeMap;

use serde::Serialize;

use crate::model as model;

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct Document {
    pub physicalKeyNames: BTreeMap<ScanCodeKey, String>,
    pub physicalToVirtual: BTreeMap<ScanCodeKey, VirtualKeyValue>
}

impl Document {
    pub fn from_model(keyboard_desc: &model::KeyboardDesc) -> Document {
        let mut physical_key_names = BTreeMap::new();
        for (scan_code, physical_key) in &keyboard_desc.physical_keys {
            if physical_key.name.is_none() { continue; }
            physical_key_names.insert(
                ScanCodeKey(*scan_code),
                physical_key.name.clone().unwrap());
        }

        let mut physical_to_virtual = BTreeMap::new();
        for (scan_code, physical_key) in &keyboard_desc.physical_keys {
            physical_to_virtual.insert(
                ScanCodeKey(*scan_code),
                VirtualKeyValue(physical_key.virtual_key));
        }

        Document {
            physicalKeyNames: physical_key_names,
            physicalToVirtual: physical_to_virtual
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct ScanCodeKey(model::ScanCode);

impl Serialize for ScanCodeKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let str = match self.0 {
            model::ScanCode::Unescaped(code) => format!("{:02X}", code),
            model::ScanCode::Extended0(code) => format!("E0{:02X}", code),
            model::ScanCode::Extended1(code) => format!("E1{:02X}", code),
        };
        serializer.serialize_str(&str)
    }
}

struct VirtualKeyValue(crate::model::VirtualKey);

impl Serialize for VirtualKeyValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        if self.0 == crate::model::VirtualKey::NONE {
            serializer.serialize_none()
        }
        else if let Some(enum_name) = self.0.to_vk_enum(true) {
            serializer.serialize_str(&enum_name)
        }
        else {
            serializer.serialize_u8(self.0.code)
        }
    }
}
