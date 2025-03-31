#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct VirtualKey {
    pub code: u8
}

impl VirtualKey {
    pub fn from_extended_bits(value: u16) -> (VirtualKey, VirtualKeyFlags) {
        (
            VirtualKey { code: (value & 0xFF) as u8 },
            VirtualKeyFlags::from_bits((value >> 8) as u8)
        )
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct VirtualKeyFlags {
    pub extended: bool, // KBDEXT
    pub multi_vk: bool, // KBDMULTIVK
    pub special: bool, // KBDSPECIAL
    pub numpad: bool, // KBDNUMPAD
    pub unicode: bool, // KBDUNICODE
    pub injected_vk: bool, // KBDINJECTEDVK
    pub mapped_vk: bool, // KBDMAPPEDVK
    pub r#break: bool, // KBDBREAK
}

impl VirtualKeyFlags {
    pub fn from_bits(flags: u8) -> Self {
        let flags = flags as u32;
        Self {
            extended: (flags & 0x01) != 0,
            multi_vk: (flags & 0x02) != 0,
            special: (flags & 0x04) != 0,
            numpad: (flags & 0x08) != 0,
            unicode: (flags & 0x10) != 0,
            injected_vk: (flags & 0x20) != 0,
            mapped_vk: (flags & 0x40) != 0,
            r#break: (flags & 0x80) != 0,
        }
    }
}

#[allow(dead_code)]
impl VirtualKey {
    // See https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes

    pub const SHIFT: Self = Self { code: 0x10 };
    pub const CONTROL: Self = Self { code: 0x11 };
    pub const ALT: Self = Self { code: 0x12 };

    pub const LEFT_WIN: Self = Self { code: 0x5B };
    pub const RIGHT_WIN: Self = Self { code: 0x5C };

    pub const F1: Self = Self { code: 0x70 };
    pub const F2: Self = Self { code: 0x71 };
    pub const F3: Self = Self { code: 0x72 };
    pub const F4: Self = Self { code: 0x73 };
    pub const F5: Self = Self { code: 0x74 };
    pub const F6: Self = Self { code: 0x75 };
    pub const F7: Self = Self { code: 0x76 };
    pub const F8: Self = Self { code: 0x77 };
    pub const F9: Self = Self { code: 0x78 };
    pub const F10: Self = Self { code: 0x79 };
    pub const F11: Self = Self { code: 0x7A };
    pub const F12: Self = Self { code: 0x7B };

    pub const LEFT_SHIFT: Self = Self { code: 0xA0 };
    pub const RIGHT_SHIFT: Self = Self { code: 0xA1 };
    pub const LEFT_CONTROL: Self = Self { code: 0xA2 };
    pub const RIGHT_CONTROL: Self = Self { code: 0xA3 };
    pub const LEFT_ALT: Self = Self { code: 0xA4 };
    pub const RIGHT_ALT: Self = Self { code: 0xA5 };

    pub const NONE: Self = Self { code: 0xFF };
}

use bimap::BiHashMap;
use lazy_static::lazy_static;


impl VirtualKey {
    pub fn to_vk_enum(&self, include_ascii: bool) -> Option<String> {
        if include_ascii && Self::is_code_ascii(self.code) {
            let mut vk_name = "VK_".to_string();
            vk_name.push(self.code as char);
            return Some(vk_name);
        }
        code_to_vk_name.get_by_left(&self.code).map(|name| name.to_string())
    }

    pub fn from_vk_enum(name: &str, include_ascii: bool) -> Option<Self> {
        if include_ascii && name.starts_with("VK_") && name.len() == 4 {
            let c = name[3..].chars().next()? as u32;
            if c < 0x80 && Self::is_code_ascii(c as u8) {
                return Some(VirtualKey { code: c as u8 });
            }
        }

        code_to_vk_name.get_by_right(&name).copied().map(|code| VirtualKey { code })
    }

    pub fn is_code_ascii(code: u8) -> bool {
        if code >= 0x30 && code <= 0x39 {
            true // Numeric keys (0-9)
        }
        else if code >= 0x41 && code <= 0x5A {
            true // Alphabetic keys (A-Z)
        }
        else {
            false // Other keys
        }
    }
}

lazy_static! {
    static ref code_to_vk_name: BiHashMap<u8, &'static str> = {
        let mut map = BiHashMap::new();
        map.insert(0x01, "VK_LBUTTON");
        map.insert(0x02, "VK_RBUTTON");
        map.insert(0x03, "VK_CANCEL");
        map.insert(0x04, "VK_MBUTTON");
        map.insert(0x05, "VK_XBUTTON1");
        map.insert(0x06, "VK_XBUTTON2");
        map.insert(0x08, "VK_BACK");
        map.insert(0x09, "VK_TAB");
        map.insert(0x0C, "VK_CLEAR");
        map.insert(0x0D, "VK_RETURN");
        map.insert(0x10, "VK_SHIFT");
        map.insert(0x11, "VK_CONTROL");
        map.insert(0x12, "VK_MENU");
        map.insert(0x13, "VK_PAUSE");
        map.insert(0x14, "VK_CAPITAL");
        map.insert(0x15, "VK_KANA");
        map.insert(0x15, "VK_HANGUL");
        map.insert(0x16, "VK_IME_ON");
        map.insert(0x17, "VK_JUNJA");
        map.insert(0x18, "VK_FINAL");
        map.insert(0x19, "VK_HANJA");
        map.insert(0x19, "VK_KANJI");
        map.insert(0x1A, "VK_IME_OFF");
        map.insert(0x1B, "VK_ESCAPE");
        map.insert(0x1C, "VK_CONVERT");
        map.insert(0x1D, "VK_NONCONVERT");
        map.insert(0x1E, "VK_ACCEPT");
        map.insert(0x1F, "VK_MODECHANGE");
        map.insert(0x20, "VK_SPACE");
        map.insert(0x21, "VK_PRIOR");
        map.insert(0x22, "VK_NEXT");
        map.insert(0x23, "VK_END");
        map.insert(0x24, "VK_HOME");
        map.insert(0x25, "VK_LEFT");
        map.insert(0x26, "VK_UP");
        map.insert(0x27, "VK_RIGHT");
        map.insert(0x28, "VK_DOWN");
        map.insert(0x29, "VK_SELECT");
        map.insert(0x2A, "VK_PRINT");
        map.insert(0x2B, "VK_EXECUTE");
        map.insert(0x2C, "VK_SNAPSHOT");
        map.insert(0x2D, "VK_INSERT");
        map.insert(0x2E, "VK_DELETE");
        map.insert(0x2F, "VK_HELP");
        map.insert(0x5B, "VK_LWIN");
        map.insert(0x5C, "VK_RWIN");
        map.insert(0x5D, "VK_APPS");
        map.insert(0x5F, "VK_SLEEP");
        map.insert(0x60, "VK_NUMPAD0");
        map.insert(0x61, "VK_NUMPAD1");
        map.insert(0x62, "VK_NUMPAD2");
        map.insert(0x63, "VK_NUMPAD3");
        map.insert(0x64, "VK_NUMPAD4");
        map.insert(0x65, "VK_NUMPAD5");
        map.insert(0x66, "VK_NUMPAD6");
        map.insert(0x67, "VK_NUMPAD7");
        map.insert(0x68, "VK_NUMPAD8");
        map.insert(0x69, "VK_NUMPAD9");
        map.insert(0x6A, "VK_MULTIPLY");
        map.insert(0x6B, "VK_ADD");
        map.insert(0x6C, "VK_SEPARATOR");
        map.insert(0x6D, "VK_SUBTRACT");
        map.insert(0x6E, "VK_DECIMAL");
        map.insert(0x6F, "VK_DIVIDE");
        map.insert(0x70, "VK_F1");
        map.insert(0x71, "VK_F2");
        map.insert(0x72, "VK_F3");
        map.insert(0x73, "VK_F4");
        map.insert(0x74, "VK_F5");
        map.insert(0x75, "VK_F6");
        map.insert(0x76, "VK_F7");
        map.insert(0x77, "VK_F8");
        map.insert(0x78, "VK_F9");
        map.insert(0x79, "VK_F10");
        map.insert(0x7A, "VK_F11");
        map.insert(0x7B, "VK_F12");
        map.insert(0x7C, "VK_F13");
        map.insert(0x7D, "VK_F14");
        map.insert(0x7E, "VK_F15");
        map.insert(0x7F, "VK_F16");
        map.insert(0x80, "VK_F17");
        map.insert(0x81, "VK_F18");
        map.insert(0x82, "VK_F19");
        map.insert(0x83, "VK_F20");
        map.insert(0x84, "VK_F21");
        map.insert(0x85, "VK_F22");
        map.insert(0x86, "VK_F23");
        map.insert(0x87, "VK_F24");
        map.insert(0x90, "VK_NUMLOCK");
        map.insert(0x91, "VK_SCROLL");
        map.insert(0xA0, "VK_LSHIFT");
        map.insert(0xA1, "VK_RSHIFT");
        map.insert(0xA2, "VK_LCONTROL");
        map.insert(0xA3, "VK_RCONTROL");
        map.insert(0xA4, "VK_LMENU");
        map.insert(0xA5, "VK_RMENU");
        map.insert(0xA6, "VK_BROWSER_BACK");
        map.insert(0xA7, "VK_BROWSER_FORWARD");
        map.insert(0xA8, "VK_BROWSER_REFRESH");
        map.insert(0xA9, "VK_BROWSER_STOP");
        map.insert(0xAA, "VK_BROWSER_SEARCH");
        map.insert(0xAB, "VK_BROWSER_FAVORITES");
        map.insert(0xAC, "VK_BROWSER_HOME");
        map.insert(0xAD, "VK_VOLUME_MUTE");
        map.insert(0xAE, "VK_VOLUME_DOWN");
        map.insert(0xAF, "VK_VOLUME_UP");
        map.insert(0xB0, "VK_MEDIA_NEXT_TRACK");
        map.insert(0xB1, "VK_MEDIA_PREV_TRACK");
        map.insert(0xB2, "VK_MEDIA_STOP");
        map.insert(0xB3, "VK_MEDIA_PLAY_PAUSE");
        map.insert(0xB4, "VK_LAUNCH_MAIL");
        map.insert(0xB5, "VK_LAUNCH_MEDIA_SELECT");
        map.insert(0xB6, "VK_LAUNCH_APP1");
        map.insert(0xB7, "VK_LAUNCH_APP2");
        map.insert(0xBA, "VK_OEM_1");
        map.insert(0xBB, "VK_OEM_PLUS");
        map.insert(0xBC, "VK_OEM_COMMA");
        map.insert(0xBD, "VK_OEM_MINUS");
        map.insert(0xBE, "VK_OEM_PERIOD");
        map.insert(0xBF, "VK_OEM_2");
        map.insert(0xC0, "VK_OEM_3");
        map.insert(0xC1, "VK_ABNT_C1");
        map.insert(0xC2, "VK_ABNT_C2");
        map.insert(0xDB, "VK_OEM_4");
        map.insert(0xDC, "VK_OEM_5");
        map.insert(0xDD, "VK_OEM_6");
        map.insert(0xDE, "VK_OEM_7");
        map.insert(0xDF, "VK_OEM_8");
        map.insert(0xE2, "VK_OEM_102");
        map.insert(0xE5, "VK_PROCESSKEY");
        map.insert(0xE7, "VK_PACKET");
        map.insert(0xF6, "VK_ATTN");
        map.insert(0xF7, "VK_CRSEL");
        map.insert(0xF8, "VK_EXSEL");
        map.insert(0xF9, "VK_EREOF");
        map.insert(0xFA, "VK_PLAY");
        map.insert(0xFB, "VK_ZOOM");
        map.insert(0xFC, "VK_NONAME");
        map.insert(0xFD, "VK_PA1");
        map.insert(0xFE, "VK_OEM_CLEAR");
        map.insert(0xFF, "VK__none_");
        map
    };
}