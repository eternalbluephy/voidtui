use crate::style::rgb::RGB;

pub struct Palette<const N: usize> {
    pub colors: [RGB; N],
}

impl<const N: usize> Palette<N> {
    /// Find the nearest color in the palette, returns the index.
    pub fn nearest(&self, color: RGB) -> usize {
        self.colors
            .iter()
            .enumerate()
            .min_by(|(_, color1), (_, color2)| {
                color1
                    .distance_to(color)
                    .partial_cmp(&color2.distance_to(color))
                    .unwrap()
            })
            .map(|(i, _)| i)
            .unwrap()
    }
}

pub const LEGACY_WINDOWS_PALETTE: Palette<16> = Palette {
    colors: [
        RGB::from_rgb(12, 12, 12),
        RGB::from_rgb(197, 15, 31),
        RGB::from_rgb(19, 161, 14),
        RGB::from_rgb(193, 156, 0),
        RGB::from_rgb(0, 55, 218),
        RGB::from_rgb(136, 23, 152),
        RGB::from_rgb(58, 150, 221),
        RGB::from_rgb(204, 204, 204),
        RGB::from_rgb(118, 118, 118),
        RGB::from_rgb(231, 72, 86),
        RGB::from_rgb(22, 198, 12),
        RGB::from_rgb(249, 241, 165),
        RGB::from_rgb(59, 120, 255),
        RGB::from_rgb(180, 0, 158),
        RGB::from_rgb(97, 214, 214),
        RGB::from_rgb(242, 242, 242),
    ],
};

pub const STANDARD_PALETTE: Palette<16> = Palette {
    colors: [
        RGB::from_rgb(0, 0, 0),
        RGB::from_rgb(170, 0, 0),
        RGB::from_rgb(0, 170, 0),
        RGB::from_rgb(170, 85, 0),
        RGB::from_rgb(0, 0, 170),
        RGB::from_rgb(170, 0, 170),
        RGB::from_rgb(0, 170, 170),
        RGB::from_rgb(170, 170, 170),
        RGB::from_rgb(85, 85, 85),
        RGB::from_rgb(255, 85, 85),
        RGB::from_rgb(85, 255, 85),
        RGB::from_rgb(255, 255, 85),
        RGB::from_rgb(85, 85, 255),
        RGB::from_rgb(255, 85, 255),
        RGB::from_rgb(85, 255, 255),
        RGB::from_rgb(255, 255, 255),
    ],
};

pub const EIGHT_BIT_PALETTE: Palette<256> = Palette {
    colors: [
        RGB::from_rgb(0, 0, 0),
        RGB::from_rgb(128, 0, 0),
        RGB::from_rgb(0, 128, 0),
        RGB::from_rgb(128, 128, 0),
        RGB::from_rgb(0, 0, 128),
        RGB::from_rgb(128, 0, 128),
        RGB::from_rgb(0, 128, 128),
        RGB::from_rgb(192, 192, 192),
        RGB::from_rgb(128, 128, 128),
        RGB::from_rgb(255, 0, 0),
        RGB::from_rgb(0, 255, 0),
        RGB::from_rgb(255, 255, 0),
        RGB::from_rgb(0, 0, 255),
        RGB::from_rgb(255, 0, 255),
        RGB::from_rgb(0, 255, 255),
        RGB::from_rgb(255, 255, 255),
        RGB::from_rgb(0, 0, 0),
        RGB::from_rgb(0, 0, 95),
        RGB::from_rgb(0, 0, 135),
        RGB::from_rgb(0, 0, 175),
        RGB::from_rgb(0, 0, 215),
        RGB::from_rgb(0, 0, 255),
        RGB::from_rgb(0, 95, 0),
        RGB::from_rgb(0, 95, 95),
        RGB::from_rgb(0, 95, 135),
        RGB::from_rgb(0, 95, 175),
        RGB::from_rgb(0, 95, 215),
        RGB::from_rgb(0, 95, 255),
        RGB::from_rgb(0, 135, 0),
        RGB::from_rgb(0, 135, 95),
        RGB::from_rgb(0, 135, 135),
        RGB::from_rgb(0, 135, 175),
        RGB::from_rgb(0, 135, 215),
        RGB::from_rgb(0, 135, 255),
        RGB::from_rgb(0, 175, 0),
        RGB::from_rgb(0, 175, 95),
        RGB::from_rgb(0, 175, 135),
        RGB::from_rgb(0, 175, 175),
        RGB::from_rgb(0, 175, 215),
        RGB::from_rgb(0, 175, 255),
        RGB::from_rgb(0, 215, 0),
        RGB::from_rgb(0, 215, 95),
        RGB::from_rgb(0, 215, 135),
        RGB::from_rgb(0, 215, 175),
        RGB::from_rgb(0, 215, 215),
        RGB::from_rgb(0, 215, 255),
        RGB::from_rgb(0, 255, 0),
        RGB::from_rgb(0, 255, 95),
        RGB::from_rgb(0, 255, 135),
        RGB::from_rgb(0, 255, 175),
        RGB::from_rgb(0, 255, 215),
        RGB::from_rgb(0, 255, 255),
        RGB::from_rgb(95, 0, 0),
        RGB::from_rgb(95, 0, 95),
        RGB::from_rgb(95, 0, 135),
        RGB::from_rgb(95, 0, 175),
        RGB::from_rgb(95, 0, 215),
        RGB::from_rgb(95, 0, 255),
        RGB::from_rgb(95, 95, 0),
        RGB::from_rgb(95, 95, 95),
        RGB::from_rgb(95, 95, 135),
        RGB::from_rgb(95, 95, 175),
        RGB::from_rgb(95, 95, 215),
        RGB::from_rgb(95, 95, 255),
        RGB::from_rgb(95, 135, 0),
        RGB::from_rgb(95, 135, 95),
        RGB::from_rgb(95, 135, 135),
        RGB::from_rgb(95, 135, 175),
        RGB::from_rgb(95, 135, 215),
        RGB::from_rgb(95, 135, 255),
        RGB::from_rgb(95, 175, 0),
        RGB::from_rgb(95, 175, 95),
        RGB::from_rgb(95, 175, 135),
        RGB::from_rgb(95, 175, 175),
        RGB::from_rgb(95, 175, 215),
        RGB::from_rgb(95, 175, 255),
        RGB::from_rgb(95, 215, 0),
        RGB::from_rgb(95, 215, 95),
        RGB::from_rgb(95, 215, 135),
        RGB::from_rgb(95, 215, 175),
        RGB::from_rgb(95, 215, 215),
        RGB::from_rgb(95, 215, 255),
        RGB::from_rgb(95, 255, 0),
        RGB::from_rgb(95, 255, 95),
        RGB::from_rgb(95, 255, 135),
        RGB::from_rgb(95, 255, 175),
        RGB::from_rgb(95, 255, 215),
        RGB::from_rgb(95, 255, 255),
        RGB::from_rgb(135, 0, 0),
        RGB::from_rgb(135, 0, 95),
        RGB::from_rgb(135, 0, 135),
        RGB::from_rgb(135, 0, 175),
        RGB::from_rgb(135, 0, 215),
        RGB::from_rgb(135, 0, 255),
        RGB::from_rgb(135, 95, 0),
        RGB::from_rgb(135, 95, 95),
        RGB::from_rgb(135, 95, 135),
        RGB::from_rgb(135, 95, 175),
        RGB::from_rgb(135, 95, 215),
        RGB::from_rgb(135, 95, 255),
        RGB::from_rgb(135, 135, 0),
        RGB::from_rgb(135, 135, 95),
        RGB::from_rgb(135, 135, 135),
        RGB::from_rgb(135, 135, 175),
        RGB::from_rgb(135, 135, 215),
        RGB::from_rgb(135, 135, 255),
        RGB::from_rgb(135, 175, 0),
        RGB::from_rgb(135, 175, 95),
        RGB::from_rgb(135, 175, 135),
        RGB::from_rgb(135, 175, 175),
        RGB::from_rgb(135, 175, 215),
        RGB::from_rgb(135, 175, 255),
        RGB::from_rgb(135, 215, 0),
        RGB::from_rgb(135, 215, 95),
        RGB::from_rgb(135, 215, 135),
        RGB::from_rgb(135, 215, 175),
        RGB::from_rgb(135, 215, 215),
        RGB::from_rgb(135, 215, 255),
        RGB::from_rgb(135, 255, 0),
        RGB::from_rgb(135, 255, 95),
        RGB::from_rgb(135, 255, 135),
        RGB::from_rgb(135, 255, 175),
        RGB::from_rgb(135, 255, 215),
        RGB::from_rgb(135, 255, 255),
        RGB::from_rgb(175, 0, 0),
        RGB::from_rgb(175, 0, 95),
        RGB::from_rgb(175, 0, 135),
        RGB::from_rgb(175, 0, 175),
        RGB::from_rgb(175, 0, 215),
        RGB::from_rgb(175, 0, 255),
        RGB::from_rgb(175, 95, 0),
        RGB::from_rgb(175, 95, 95),
        RGB::from_rgb(175, 95, 135),
        RGB::from_rgb(175, 95, 175),
        RGB::from_rgb(175, 95, 215),
        RGB::from_rgb(175, 95, 255),
        RGB::from_rgb(175, 135, 0),
        RGB::from_rgb(175, 135, 95),
        RGB::from_rgb(175, 135, 135),
        RGB::from_rgb(175, 135, 175),
        RGB::from_rgb(175, 135, 215),
        RGB::from_rgb(175, 135, 255),
        RGB::from_rgb(175, 175, 0),
        RGB::from_rgb(175, 175, 95),
        RGB::from_rgb(175, 175, 135),
        RGB::from_rgb(175, 175, 175),
        RGB::from_rgb(175, 175, 215),
        RGB::from_rgb(175, 175, 255),
        RGB::from_rgb(175, 215, 0),
        RGB::from_rgb(175, 215, 95),
        RGB::from_rgb(175, 215, 135),
        RGB::from_rgb(175, 215, 175),
        RGB::from_rgb(175, 215, 215),
        RGB::from_rgb(175, 215, 255),
        RGB::from_rgb(175, 255, 0),
        RGB::from_rgb(175, 255, 95),
        RGB::from_rgb(175, 255, 135),
        RGB::from_rgb(175, 255, 175),
        RGB::from_rgb(175, 255, 215),
        RGB::from_rgb(175, 255, 255),
        RGB::from_rgb(215, 0, 0),
        RGB::from_rgb(215, 0, 95),
        RGB::from_rgb(215, 0, 135),
        RGB::from_rgb(215, 0, 175),
        RGB::from_rgb(215, 0, 215),
        RGB::from_rgb(215, 0, 255),
        RGB::from_rgb(215, 95, 0),
        RGB::from_rgb(215, 95, 95),
        RGB::from_rgb(215, 95, 135),
        RGB::from_rgb(215, 95, 175),
        RGB::from_rgb(215, 95, 215),
        RGB::from_rgb(215, 95, 255),
        RGB::from_rgb(215, 135, 0),
        RGB::from_rgb(215, 135, 95),
        RGB::from_rgb(215, 135, 135),
        RGB::from_rgb(215, 135, 175),
        RGB::from_rgb(215, 135, 215),
        RGB::from_rgb(215, 135, 255),
        RGB::from_rgb(215, 175, 0),
        RGB::from_rgb(215, 175, 95),
        RGB::from_rgb(215, 175, 135),
        RGB::from_rgb(215, 175, 175),
        RGB::from_rgb(215, 175, 215),
        RGB::from_rgb(215, 175, 255),
        RGB::from_rgb(215, 215, 0),
        RGB::from_rgb(215, 215, 95),
        RGB::from_rgb(215, 215, 135),
        RGB::from_rgb(215, 215, 175),
        RGB::from_rgb(215, 215, 215),
        RGB::from_rgb(215, 215, 255),
        RGB::from_rgb(215, 255, 0),
        RGB::from_rgb(215, 255, 95),
        RGB::from_rgb(215, 255, 135),
        RGB::from_rgb(215, 255, 175),
        RGB::from_rgb(215, 255, 215),
        RGB::from_rgb(215, 255, 255),
        RGB::from_rgb(255, 0, 0),
        RGB::from_rgb(255, 0, 95),
        RGB::from_rgb(255, 0, 135),
        RGB::from_rgb(255, 0, 175),
        RGB::from_rgb(255, 0, 215),
        RGB::from_rgb(255, 0, 255),
        RGB::from_rgb(255, 95, 0),
        RGB::from_rgb(255, 95, 95),
        RGB::from_rgb(255, 95, 135),
        RGB::from_rgb(255, 95, 175),
        RGB::from_rgb(255, 95, 215),
        RGB::from_rgb(255, 95, 255),
        RGB::from_rgb(255, 135, 0),
        RGB::from_rgb(255, 135, 95),
        RGB::from_rgb(255, 135, 135),
        RGB::from_rgb(255, 135, 175),
        RGB::from_rgb(255, 135, 215),
        RGB::from_rgb(255, 135, 255),
        RGB::from_rgb(255, 175, 0),
        RGB::from_rgb(255, 175, 95),
        RGB::from_rgb(255, 175, 135),
        RGB::from_rgb(255, 175, 175),
        RGB::from_rgb(255, 175, 215),
        RGB::from_rgb(255, 175, 255),
        RGB::from_rgb(255, 215, 0),
        RGB::from_rgb(255, 215, 95),
        RGB::from_rgb(255, 215, 135),
        RGB::from_rgb(255, 215, 175),
        RGB::from_rgb(255, 215, 215),
        RGB::from_rgb(255, 215, 255),
        RGB::from_rgb(255, 255, 0),
        RGB::from_rgb(255, 255, 95),
        RGB::from_rgb(255, 255, 135),
        RGB::from_rgb(255, 255, 175),
        RGB::from_rgb(255, 255, 215),
        RGB::from_rgb(255, 255, 255),
        RGB::from_rgb(8, 8, 8),
        RGB::from_rgb(18, 18, 18),
        RGB::from_rgb(28, 28, 28),
        RGB::from_rgb(38, 38, 38),
        RGB::from_rgb(48, 48, 48),
        RGB::from_rgb(58, 58, 58),
        RGB::from_rgb(68, 68, 68),
        RGB::from_rgb(78, 78, 78),
        RGB::from_rgb(88, 88, 88),
        RGB::from_rgb(98, 98, 98),
        RGB::from_rgb(108, 108, 108),
        RGB::from_rgb(118, 118, 118),
        RGB::from_rgb(128, 128, 128),
        RGB::from_rgb(138, 138, 138),
        RGB::from_rgb(148, 148, 148),
        RGB::from_rgb(158, 158, 158),
        RGB::from_rgb(168, 168, 168),
        RGB::from_rgb(178, 178, 178),
        RGB::from_rgb(188, 188, 188),
        RGB::from_rgb(198, 198, 198),
        RGB::from_rgb(208, 208, 208),
        RGB::from_rgb(218, 218, 218),
        RGB::from_rgb(228, 228, 228),
        RGB::from_rgb(238, 238, 238),
    ],
};
