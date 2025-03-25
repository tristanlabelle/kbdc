#[derive(PartialEq, Eq, Hash)]
pub struct VirtualKey {
    pub code: u8
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
}