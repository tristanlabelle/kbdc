use std::collections::HashMap;

mod scan_codes;
mod virtual_keys;
pub use scan_codes::*;
pub use virtual_keys::*;

pub struct KeyboardDesc {
    pub physical_keys: HashMap<ScanCode, PhysicalKeyDesc>,
    pub virtual_keys: HashMap<VirtualKey, KeyEffect>,
    pub dead_keys: HashMap<u16, DeadKeyDesc>,
}

impl KeyboardDesc {
    pub fn new() -> Self {
        Self {
            physical_keys: HashMap::new(),
            virtual_keys: HashMap::new(),
            dead_keys: HashMap::new(),
        }
    }
}

pub struct PhysicalKeyDesc {
    /// The virtual key to which the scan code maps.
    pub virtual_key: VirtualKey,
    // TODO: Virtual key flags
    /// The name of the physical key.
    pub name: Option<String>,
}

pub struct KeyModifiers {
    /// The shift modifier.
    pub shift: bool, // KBDSHIFT
    /// The control modifier. Combines with alt for altgr.
    pub control: bool, // KBDCTRL
    /// The alt modifier. Combines with control for altgr.
    pub alt: bool, // KBDALT
    /// The kana modifier.
    pub kana: bool, // KBDKANA
    pub roya: bool, // KBDROYA
    pub loya: bool, // KBDLOYA
    pub unknown0x40: bool,
    pub grpseltap: bool, // KBDGRPSELTAP
}

pub enum KeyEffect {
    /// The virtual key is used as a modifier.
    Modifier(KeyModifiers),
    /// The virtual key is used to type characters.
    Typing(KeyTyping)
}

pub struct KeyTyping {
    /// Maps modifiers to the typing effect.
    pub by_modifiers: HashMap<KeyModifiers, TypingEffect>,

    /// Interpret caps lock as a shift modifier.
    pub caps_lock_means_shift: bool, // CAPLOK
    /// Caps lock upper cases the unshifted char.
    pub caps_lock_uppercases: bool, // SGCAPS
    /// Interpret caps lock as a shift modifier when altgr is pressed.
    pub caps_lock_altgr_means_shift: bool, // CAPLOKALTGR
    /// Kana lock is supported for this key.
    pub kana_support: bool, // KANALOK
    /// grpseltap is supported for this key.
    pub grpseltap_support: bool, // GRPSELTAP
}

pub enum TypingEffect {
    /// A character gets typed.
    Char(u16),
    /// A dead key gets triggered.
    DeadKey(u16),
    /// A ligature gets entered.
    Ligature
}

pub struct DeadKeyDesc {
    /// The human-readable display name of this dead key.
    pub name: Option<String>,
    /// Maps base chars to a dead key combination.
    pub combos: HashMap<u16, DeadKeyCombo>
}

pub struct DeadKeyCombo {
    // The character resulting from the dead key + character typed.
    pub composed_char: u16,
    pub flags: u16
}