use unicode_width::UnicodeWidthChar;

use crate::geometry::size::Size;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    pub raw: String,
    pub size: Size,
}

impl Text {
    pub fn new(raw: impl Into<String>) -> Self {
        let raw = raw.into();
        let size = Self::size_of(&raw);
        Self { raw, size }
    }

    pub fn raw(&self) -> &String {
        &self.raw
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn set(&mut self, raw: impl Into<String>) {
        self.raw = raw.into();
        self.size = Self::size_of(&self.raw);
    }

    pub fn size_of(content: impl AsRef<str>) -> Size<u16> {
        let mut size = Size::new(0, 0);

        for line in content.as_ref().lines() {
            let mut width = 0;
            size.height += 1;
            for char in line.chars() {
                width += char.width().unwrap();
            }
            size.width = size.width.max(width as u16);
        }

        size
    }
}
