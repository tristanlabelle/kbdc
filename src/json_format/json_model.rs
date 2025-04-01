
use std::collections::BTreeMap;

use serde::Serialize;

use crate::model as model;

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct Document {
    physicalKeyNames: BTreeMap<ScanCodeKey, String>,
    physicalToVirtual: BTreeMap<ScanCodeKey, VirtualKeyValue>,
    deadKeys: BTreeMap<char, DeadKeyDesc>
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

        let mut dead_keys = BTreeMap::new();
        for (char, dead_key) in &keyboard_desc.dead_keys {
            dead_keys.insert(
                char::from_u32(*char as u32).unwrap(),
                DeadKeyDesc::from_model(&dead_key)
            );
        }

        Document {
            physicalKeyNames: physical_key_names,
            physicalToVirtual: physical_to_virtual,
            deadKeys: dead_keys
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

#[derive(Serialize)]
#[allow(non_snake_case)]
struct DeadKeyDesc {
    name: Option<String>,
    combos: BTreeMap<char, char>
}

impl DeadKeyDesc {
    fn from_model(value: &model::DeadKeyDesc) -> Self {
        let mut combos = BTreeMap::new();
        for (char, combo) in &value.combos {
            combos.insert(
                char::from_u32(*char as u32).unwrap(),
                char::from_u32(combo.composed_char as u32).unwrap()
            );
        }

        Self {
            name: value.name.clone(),
            combos: combos
        }
    }
}