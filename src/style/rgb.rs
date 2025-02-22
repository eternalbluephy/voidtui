use super::{
    color::ColorSystem,
    palette::{EIGHT_BIT_PALETTE, LEGACY_WINDOWS_PALETTE, STANDARD_PALETTE},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }

    pub const fn from_hex(hex: u32) -> Self {
        RGB {
            r: (hex >> 16 & 0xff) as u8,
            g: (hex >> 8 & 0xff) as u8,
            b: (hex & 0xff) as u8,
        }
    }

    /// Get the normalized (r, g, b, a), all values ranges from 0 to 1.
    pub const fn normalized(&self) -> (f64, f64, f64) {
        (
            self.r as f64 / 255.0,
            self.g as f64 / 255.0,
            self.b as f64 / 255.0,
        )
    }

    /// Calculate the relative luminance of the color.
    /// Reference: https://www.w3.org/TR/2008/REC-WCAG20-20081211/#relativeluminancedef.
    pub fn luminance(&self) -> f64 {
        let (r, g, b) = self.normalized();
        let process = |v: f64| {
            if v <= 0.03928 {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        };
        let (r, g, b) = (process(r), process(g), process(b));
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// Calculate the contrast ratios between two colors.
    /// Contrast ratios can range from 1 to 21 (commonly written 1:1 to 21:1).
    /// Reference: https://www.w3.org/TR/2008/REC-WCAG20-20081211/#contrast-ratiodef.
    pub fn contrast(lhs: Self, rhs: Self) -> f64 {
        let (a, b) = (lhs.luminance(), rhs.luminance());
        (f64::max(a, b) + 0.05) / (f64::min(a, b) + 0.05)
    }

    /// Translate the color into its appearance in a specified transparency level
    /// on a specified background.
    pub fn alpha_on(&mut self, alpha: u8, background: Self) -> &mut Self {
        let alpha = alpha as f32 / 255.0;
        let inv_alpha = 1.0 - alpha;
        self.r = (self.r as f32 * alpha + background.r as f32 * inv_alpha).round() as u8;
        self.g = (self.g as f32 * alpha + background.g as f32 * inv_alpha).round() as u8;
        self.b = (self.b as f32 * alpha + background.b as f32 * inv_alpha).round() as u8;
        self
    }

    /// Get the color distance(aka. similarity) to another color.
    pub fn distance_to(&self, color: RGB) -> f64 {
        let (r1, g1, b1) = (self.r as f64, self.g as f64, self.b as f64);
        let (r2, g2, b2) = (color.r as f64, color.g as f64, color.b as f64);
        let rmean = (r1 + r2) / 2.0;
        let r = r1 - r2;
        let g = g1 - g2;
        let b = b1 - b2;
        ((2.0 + rmean / 256.0) * r * r + 4.0 * g * g + (2.0 + (255.0 - rmean) / 256.0) * b * b)
            .sqrt()
    }

    pub fn ansi_codes(&self, system: ColorSystem, foreground: bool) -> String {
        match system {
            ColorSystem::Disabled => String::new(),
            ColorSystem::LegacyWindows => self.legacy_windows_ansi_codes(foreground),
            ColorSystem::Standard => self.standard_ansi_codes(foreground),
            ColorSystem::EightBit => self.eight_bit_ansi_colors(foreground),
            ColorSystem::TrueColor => self.true_color_ansi_codes(foreground),
        }
    }

    fn legacy_windows_ansi_codes(&self, foreground: bool) -> String {
        let number = LEGACY_WINDOWS_PALETTE.nearest(*self);
        let (fore, back) = if number < 8 { (30, 40) } else { (82, 92) };
        (if foreground {
            fore + number
        } else {
            back + number
        })
        .to_string()
    }

    fn standard_ansi_codes(&self, foreground: bool) -> String {
        let number = STANDARD_PALETTE.nearest(*self);
        let (fore, back) = if number < 8 { (30, 40) } else { (82, 92) };
        (if foreground {
            fore + number
        } else {
            back + number
        })
        .to_string()
    }

    fn eight_bit_ansi_colors(&self, foreground: bool) -> String {
        let number = EIGHT_BIT_PALETTE.nearest(*self);
        format!("{};5;{}", if foreground { "38" } else { "48" }, number)
    }

    fn true_color_ansi_codes(&self, foreground: bool) -> String {
        format!(
            "{};2;{};{};{}",
            if foreground { "38" } else { "48" },
            self.r,
            self.g,
            self.b
        )
    }
}

impl From<(u8, u8, u8)> for RGB {
    fn from(value: (u8, u8, u8)) -> Self {
        RGB::from_rgb(value.0, value.1, value.2)
    }
}

impl From<u32> for RGB {
    fn from(value: u32) -> Self {
        RGB::from_hex(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::style::rgb::ColorSystem;
    use crate::style::rgb::RGB;

    #[test]
    fn from_hex() {
        let color1 = RGB::from_hex(0x1f1e33);
        let color2 = RGB::from_rgb(31, 30, 51);
        assert_eq!(color1, color2);
    }

    #[test]
    fn ansi_codes() {
        let color = RGB::from_rgb(123, 219, 89);
        println!(
            "\x1b[{}m██\x1b[0m",
            color.ansi_codes(ColorSystem::Disabled, true)
        );
        println!(
            "\x1b[{}m██\x1b[0m",
            color.ansi_codes(ColorSystem::LegacyWindows, true)
        );
        println!(
            "\x1b[{}m██\x1b[0m",
            color.ansi_codes(ColorSystem::Standard, true)
        );
        println!(
            "\x1b[{}m██\x1b[0m",
            color.ansi_codes(ColorSystem::EightBit, true)
        );
        println!(
            "\x1b[{}m██\x1b[0m",
            color.ansi_codes(ColorSystem::TrueColor, true)
        );
    }
}
