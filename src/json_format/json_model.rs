
use std::collections::BTreeMap;
use crate::model as model;

#[derive(serde::Serialize)]
#[allow(non_snake_case)]
pub struct Document {
    physicalKeyNames: BTreeMap<ScanCodeKey, String>,
    physicalToVirtualKeys: BTreeMap<ScanCodeKey, VirtualKeyValue>,
    modifierKeys: BTreeMap<VirtualKeyKey, ModifierKey>,
    typingKeys: BTreeMap<VirtualKeyKey, KeyTypingDesc>,
    deadKeys: BTreeMap<char, DeadKeyDesc>,
    version: u16,
    #[serde(skip_serializing_if = "is_false")]
    supportsAltGr: bool,
    r#type: u32,
    subtype: u32,
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

        let mut physical_to_virtual_keys = BTreeMap::new();
        for (scan_code, physical_key) in &keyboard_desc.physical_keys {
            physical_to_virtual_keys.insert(
                ScanCodeKey(*scan_code),
                VirtualKeyValue(physical_key.virtual_key));
        }

        let mut modifier_keys = BTreeMap::new();
        let mut typing_keys = BTreeMap::new();
        for (virtual_key, key_effect) in &keyboard_desc.virtual_keys {
            match key_effect {
                model::KeyEffect::Modifier(key_modifiers) => {
                    if let Some(modifier_key) = ModifierKey::from_model(key_modifiers) {
                        modifier_keys.insert(
                            VirtualKeyKey(*virtual_key),
                            modifier_key);
                    }
                },
                model::KeyEffect::Typing(key_typing) => {
                    typing_keys.insert(
                        VirtualKeyKey(*virtual_key),
                        KeyTypingDesc::from_model(key_typing)
                    );
                }
            }
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
            physicalToVirtualKeys: physical_to_virtual_keys,
            modifierKeys: modifier_keys,
            typingKeys: typing_keys,
            deadKeys: dead_keys,
            version: keyboard_desc.version,
            supportsAltGr: keyboard_desc.supports_altgr,
            r#type: keyboard_desc.type_value,
            subtype: keyboard_desc.subtype_value
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct ScanCodeKey(model::ScanCode);

impl serde::Serialize for ScanCodeKey {
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct VirtualKeyKey(crate::model::VirtualKey);

impl serde::Serialize for VirtualKeyKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        if let Some(enum_name) = self.0.to_vk_enum(true) {
            serializer.serialize_str(&enum_name)
        }
        else {
            panic!("VirtualKey {:?} does not have a name", self.0)
        }
    }
}

struct VirtualKeyValue(crate::model::VirtualKey);

impl serde::Serialize for VirtualKeyValue {
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

#[allow(non_camel_case_types)]
enum ModifierKey {
    shift,
    control,
    alt,
    kana,
    roya,
    loya
}

impl ModifierKey {
    fn from_model(value: &model::KeyModifiers) -> Option<Self> {
        if value.to_bits().count_ones() != 1 { None }
        else if value.shift { Some(ModifierKey::shift) }
        else if value.control { Some(ModifierKey::control) }
        else if value.alt { Some(ModifierKey::alt) }
        else if value.kana { Some(ModifierKey::kana) }
        else if value.roya { Some(ModifierKey::roya) }
        else if value.loya { Some(ModifierKey::loya) }
        else { None }
    }
}

impl serde::Serialize for ModifierKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        match self {
            ModifierKey::shift => serializer.serialize_str("shift"),
            ModifierKey::control => serializer.serialize_str("control"),
            ModifierKey::alt => serializer.serialize_str("alt"),
            ModifierKey::kana => serializer.serialize_str("kana"),
            ModifierKey::roya => serializer.serialize_str("roya"),
            ModifierKey::loya => serializer.serialize_str("loya")
        }
    }
}

struct KeyModifiersKey(model::KeyModifiers);

impl KeyModifiersKey {
    fn to_mask_string(&self) -> String {
        let mut mask = String::new();
        if self.0.shift { mask.push_str("s"); }
        if self.0.control { mask.push_str("c"); }
        if self.0.alt { mask.push_str("a"); }
        if self.0.kana { mask.push_str("k"); }
        if self.0.roya { mask.push_str("r"); }
        if self.0.loya { mask.push_str("l"); }
        if self.0.unknown0x40 { mask.push_str("u"); }
        if self.0.grpseltap { mask.push_str("g"); }
        mask
    }
}

impl PartialEq for KeyModifiersKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(&other) == std::cmp::Ordering::Equal
    }
}

impl Eq for KeyModifiersKey {}

impl PartialOrd for KeyModifiersKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for KeyModifiersKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.to_bits().cmp(&other.0.to_bits())
    }
}

impl serde::Serialize for KeyModifiersKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.to_mask_string())
    }
}

fn is_false(b: &bool) -> bool { !b }

#[derive(serde::Serialize)]
#[allow(non_snake_case)]
struct KeyTypingDesc {
    pub byModifiers: BTreeMap<KeyModifiersKey, TypingEffect>,
    #[serde(skip_serializing_if = "is_false")]
    pub capsLockAsShift: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub capsLockAsUppercase: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub capsLockAltGrAsShift: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub kanaSupport: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub grpseltapSupport: bool
}

impl KeyTypingDesc {
    fn from_model(value: &model::KeyTyping) -> Self {
        let mut by_modifiers = BTreeMap::new();
        for (key_modifiers, effect) in &value.by_modifiers {
            match effect {
                model::TypingEffect::Char(char) => {
                    by_modifiers.insert(
                        KeyModifiersKey(*key_modifiers),
                        TypingEffect::Char(char::from_u32(*char as u32).unwrap())
                    );
                },
                model::TypingEffect::DeadKey(char) => {
                    by_modifiers.insert(
                        KeyModifiersKey(*key_modifiers),
                        TypingEffect::DeadKey {
                            deadKey: char::from_u32(*char as u32).unwrap()
                        }
                    );
                },
                model::TypingEffect::Ligature(chars) => {
                    by_modifiers.insert(
                        KeyModifiersKey(*key_modifiers),
                        TypingEffect::Ligature {
                            ligature: String::from_utf16_lossy(chars)
                        }
                    );
                }
            }
        }

        Self {
            byModifiers: by_modifiers,
            capsLockAsShift: value.caps_lock_as_shift,
            capsLockAsUppercase: value.caps_lock_as_uppercase,
            capsLockAltGrAsShift: value.caps_lock_altgr_as_shift,
            kanaSupport: value.kana_support,
            grpseltapSupport: value.grpseltap_support
        }
    }
}

#[derive(serde::Serialize)]
#[serde(untagged)]
enum TypingEffect {
    Char(char),
    DeadKey { deadKey: char },
    Ligature { ligature: String }
}

#[derive(serde::Serialize)]
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