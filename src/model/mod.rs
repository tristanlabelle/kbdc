use std::collections::HashMap;

mod scan_codes;
mod virtual_keys;
pub use scan_codes::*;
pub use virtual_keys::*;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;

pub struct KeyboardDesc {
    // pusVSCtoVK, bMaxVSCtoVK, pVSCtoVK_E0, pVSCtoVK_E1
    pub physical_keys: HashMap<ScanCode, PhysicalKeyDesc>,

    // pCharModifiers, pVkToWcharTable, pKeyNames, pKeyNamesExt
    pub virtual_keys: HashMap<VirtualKey, KeyEffect>,

    // pDeadKey, pKeyNamesDead
    pub dead_keys: HashMap<u16, DeadKeyDesc>,

    // fLocaleFlags
    pub version: u16,
    pub altgr_flag: bool, // KLLF_ALTGR
    pub shift_lock_flag: bool, // KLLF_SHIFTLOCK
    pub lrm_rlm_flag: bool, // KLLF_LRM_RLM

    // TODO: nLgMax, cbLgEntry, pLigature
    // TODO: dwType, dwSubType
}

impl KeyboardDesc {
    pub fn new() -> Self {
        Self {
            physical_keys: HashMap::new(),
            virtual_keys: HashMap::new(),
            dead_keys: HashMap::new(),
            version: 0,
            altgr_flag: false,
            shift_lock_flag: false,
            lrm_rlm_flag: false
        }
    }
}

pub struct PhysicalKeyDesc {
    /// The virtual key to which the scan code maps.
    pub virtual_key: VirtualKey,
    /// Flags applying to that virtual key.
    pub virtual_key_flags: VirtualKeyFlags,
    /// The name of the physical key.
    pub name: Option<String>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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

impl KeyModifiers {
    pub fn from_bits(flags: u8) -> Self {
        let flags = flags as u32;
        return Self {
            shift: (flags & KBDSHIFT) != 0,
            control: (flags & KBDCTRL) != 0,
            alt: (flags & KBDALT) != 0,
            kana: (flags & KBDKANA) != 0,
            roya: (flags & KBDROYA) != 0,
            loya: (flags & KBDLOYA) != 0,
            unknown0x40: (flags & 0x40) != 0,
            grpseltap: (flags & KBDGRPSELTAP) != 0,
        }
    }
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
    /// A ligature gets entered (not implemented yet).
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