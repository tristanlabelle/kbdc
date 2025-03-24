use std::collections::HashMap;

pub struct KeyboardDescriptor {
    pub key_names: HashMap<ScanCode, String>,
    pub dead_keys: Vec<DeadKey>,
}

pub struct KeyModifiers {
    pub shift: bool, // CAPOK
    pub caps_lock: bool, // SGCAPLOK
    pub altgr: bool, // CAPLOKALTGR
    pub kana: bool, // KANALOK
}

pub enum VirtualKeyEffect {
    Chars(Box<[u16]>),
    DeadKey(Box<[u16]>),
    Ligature
}

impl KeyboardDescriptor {
    pub fn new() -> Self {
        Self {
            key_names: HashMap::new(),
            dead_keys: Vec::new(),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum ScanCode {
    Unescaped(u8), // Most significant bit unused
    Extended(u8), // E0-escaped, most significant bit unused
    Pause, // E1-escaped
}

pub struct DeadKey {
    pub name: Option<String>,
    pub base_char: u16,
    pub diacritic: u16,
    pub composed_char: u16,
    pub flags: u16
}