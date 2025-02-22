use super::{rgb::RGB, theme::Theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSystem {
    Disabled,
    LegacyWindows,
    Standard,
    EightBit,
    TrueColor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Text,
    Background,
    Primary,
    Success,
    Warning,
    Danger,
    RGB(RGB),
}

impl Color {
    pub fn on_theme(&self, theme: &Theme) -> RGB {
        match *self {
            Color::Text => theme.text,
            Color::Background => theme.background,
            Color::Primary => theme.primary,
            Color::Success => theme.success,
            Color::Warning => theme.warning,
            Color::Danger => theme.danger,
            Color::RGB(rgb) => rgb,
        }
    }
}

impl From<RGB> for Color {
    fn from(rgb: RGB) -> Self {
        Color::RGB(rgb)
    }
}

impl From<u32> for Color {
    fn from(hex: u32) -> Self {
        Color::RGB(RGB::from_hex(hex))
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(rgb: (u8, u8, u8)) -> Self {
        Color::RGB(RGB::from_rgb(rgb.0, rgb.1, rgb.2))
    }
}

impl From<&str> for Color {
    fn from(name: &str) -> Self {
        match name {
            "text" => Color::Text,
            "background" => Color::Background,
            "primary" => Color::Primary,
            "success" => Color::Success,
            "warning" => Color::Warning,
            "danger" => Color::Danger,
            _ => Color::Text,
        }
    }
}
