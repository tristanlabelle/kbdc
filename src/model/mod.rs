use std::collections::HashMap;

mod scan_codes;
mod virtual_keys;
pub use scan_codes::*;
pub use virtual_keys::*;

pub struct KeyboardDescriptor {
    pub scan_codes: HashMap<ScanCode, ScanCodeEntry>,
    pub dead_keys: Vec<DeadKey>,
}

pub struct ScanCodeEntry {
    pub virtual_key: VirtualKey,
    pub name: Option<String>,
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
            scan_codes: HashMap::new(),
            dead_keys: Vec::new(),
        }
    }
}

pub struct DeadKey {
    pub name: Option<String>,
    pub base_char: u16,
    pub diacritic: u16,
    pub composed_char: u16,
    pub flags: u16
}