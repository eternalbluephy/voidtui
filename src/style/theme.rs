use crate::style::rgb::RGB;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    pub text: RGB,
    pub background: RGB,
    pub primary: RGB,
    pub success: RGB,
    pub warning: RGB,
    pub danger: RGB,
}

impl Theme {
    pub const TOKYO_NIGHT: Self = Self {
        text: RGB::from_hex(0xA9B1D6),
        background: RGB::from_hex(0x1A1B26),
        primary: RGB::from_hex(0xBB9AF7),
        success: RGB::from_hex(0x9ECE6A),
        warning: RGB::from_hex(0xE0AF68),
        danger: RGB::from_hex(0xF7768E),
    };
}
