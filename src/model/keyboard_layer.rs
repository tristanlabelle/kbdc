// Represents the data returned by the KbdLayerDescriptor
// entry point of a keyboard layout DLL.

use std::collections::{BTreeMap, HashMap};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;
use crate::model::scan_codes::*;
use crate::model::virtual_keys::*;

pub struct KeyboardDesc {
    // pusVSCtoVK, bMaxVSCtoVK, pVSCtoVK_E0, pVSCtoVK_E1
    pub physical_keys: BTreeMap<ScanCode, PhysicalKeyDesc>,

    // pCharModifiers, pVkToWcharTable, pKeyNames, pKeyNamesExt
    pub virtual_keys: HashMap<VirtualKey, KeyEffect>,

    // pDeadKey, pKeyNamesDead
    pub dead_keys: HashMap<u16, DeadKeyDesc>,

    // fLocaleFlags
    pub version: u16,

    /// The layout treats right Alt key as Control+Alt.
    pub supports_altgr: bool, // KLLF_ALTGR

    /// The layout turns off Caps Lock when Shift is depressed.
    pub supports_shift_lock: bool, // KLLF_SHIFTLOCK

    /// The layout inserts Left-to-Right/Right-to-Left Markers on some combinations.
    pub supports_directionality: bool, // KLLF_LRM_RLM

    // TODO: nLgMax, cbLgEntry, pLigature
    // TODO: dwType, dwSubType

    pub type_value: u32,
    pub subtype_value: u32,
}

impl KeyboardDesc {
    pub fn new() -> Self {
        Self {
            physical_keys: BTreeMap::new(),
            virtual_keys: HashMap::new(),
            dead_keys: HashMap::new(),
            version: 0,
            supports_altgr: false,
            supports_shift_lock: false,
            supports_directionality: false,
            type_value: 0,
            subtype_value: 0,
        }
    }

    pub const TYPE_GENERIC101: u32 = 4;
    pub const TYPE_JAPAN: u32 = 7;
    pub const TYPE_KOREA: u32 = 8;
    pub const TYPE_UNKNOWN: u32 = 0x51;
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

    pub fn to_bits(&self) -> u8 {
        let mut flags: u32 = 0;
        if self.shift { flags |= KBDSHIFT; }
        if self.control { flags |= KBDCTRL; }
        if self.alt { flags |= KBDALT; }
        if self.kana { flags |= KBDKANA; }
        if self.roya { flags |= KBDROYA; }
        if self.loya { flags |= KBDLOYA; }
        if self.unknown0x40 { flags |= 0x40; }
        if self.grpseltap { flags |= KBDGRPSELTAP; }
        return flags as u8;
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
    pub caps_lock_as_shift: bool, // CAPLOK
    /// Caps lock upper cases the unshifted char.
    pub caps_lock_as_uppercase: bool, // SGCAPS
    /// Interpret caps lock as a shift modifier when altgr is pressed.
    pub caps_lock_altgr_as_shift: bool, // CAPLOKALTGR
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
    Ligature(Box<[u16]>)
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
