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
        text: RGB::from_hex(0x9aa5ce),
        background: RGB::from_hex(0x1a1b26),
        primary: RGB::from_hex(0x2ac3de),
        success: RGB::from_hex(0x9ece6a),
        warning: RGB::from_hex(0xe0af68),
        danger: RGB::from_hex(0xf7768e),
    };
}
