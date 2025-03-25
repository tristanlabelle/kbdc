#[derive(PartialEq, Eq, Hash)]
pub enum ScanCode {
    Unescaped(u8), // Most significant bit unused
    Extended0(u8), // E0-escaped, most significant bit unused
    Extended1(u8), // E1-escaped, most significant bit unused
}

#[allow(dead_code)]
impl ScanCode {
    // See https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input

    // Numeric row
    pub const _1: Self = Self::Unescaped(0x02);
    pub const _2: Self = Self::Unescaped(0x03);
    pub const _3: Self = Self::Unescaped(0x04);
    pub const _4: Self = Self::Unescaped(0x05);
    pub const _5: Self = Self::Unescaped(0x06);
    pub const _6: Self = Self::Unescaped(0x07);
    pub const _7: Self = Self::Unescaped(0x08);
    pub const _8: Self = Self::Unescaped(0x09);
    pub const _9: Self = Self::Unescaped(0x0A);
    pub const _0: Self = Self::Unescaped(0x0B);

    // Top alphabetic row
    pub const Q: Self = Self::Unescaped(0x10);
    pub const W: Self = Self::Unescaped(0x11);
    pub const E: Self = Self::Unescaped(0x12);
    pub const R: Self = Self::Unescaped(0x13);
    pub const T: Self = Self::Unescaped(0x14);
    pub const Y: Self = Self::Unescaped(0x15);
    pub const U: Self = Self::Unescaped(0x16);
    pub const I: Self = Self::Unescaped(0x17);
    pub const O: Self = Self::Unescaped(0x18);
    pub const P: Self = Self::Unescaped(0x19);

    // Middle alphabetic row (home row)
    pub const A: Self = Self::Unescaped(0x1E);
    pub const S: Self = Self::Unescaped(0x1F);
    pub const D: Self = Self::Unescaped(0x20);
    pub const F: Self = Self::Unescaped(0x21);
    pub const G: Self = Self::Unescaped(0x22);
    pub const H: Self = Self::Unescaped(0x23);
    pub const J: Self = Self::Unescaped(0x24);
    pub const K: Self = Self::Unescaped(0x25);
    pub const L: Self = Self::Unescaped(0x26);

    // Bottom alphabetic row
    pub const Z: Self = Self::Unescaped(0x2C);
    pub const X: Self = Self::Unescaped(0x1D);
    pub const C: Self = Self::Unescaped(0x2E);
    pub const V: Self = Self::Unescaped(0x2F);
    pub const B: Self = Self::Unescaped(0x30);
    pub const N: Self = Self::Unescaped(0x31);
    pub const M: Self = Self::Unescaped(0x32);

    pub const PAUSE: Self = Self::Extended1(0x1D);
}