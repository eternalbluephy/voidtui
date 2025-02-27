use unicode_width::UnicodeWidthChar;

use crate::{
    buffer::pixel::Pixel,
    geometry::area::Area,
    style::{color::Color, color::ColorSystem, style::Style, theme::Theme},
};

#[derive(Debug, Clone)]
pub struct Buffer {
    pixels: Vec<Pixel>,
    width: u16,
    height: u16,
}

impl Buffer {
    /// Create a buffer with a specific size.
    pub fn new(width: u16, height: u16) -> Self {
        let pixels = vec![Pixel::new(); (width * height) as usize];
        Buffer {
            pixels,
            width,
            height,
        }
    }

    /// Get the width of the buffer.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Get the height of the buffer.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Get the pixel reference at (x, y).
    /// Panics if the pixel is out of bounds.
    pub fn get(&self, x: u16, y: u16) -> &Pixel {
        if x >= self.width || y >= self.height {
            panic!("Pixel out of buffer bounds: ({}, {})", x, y);
        }
        &self.pixels[(y * self.width + x) as usize]
    }

    /// Get the mutable pixel reference at (x, y).
    /// Panics if the pixel is out of bounds.
    pub fn get_mut(&mut self, x: u16, y: u16) -> &mut Pixel {
        if x >= self.width || y >= self.height {
            panic!("Pixel out of buffer bounds: ({}, {})", x, y);
        }
        &mut self.pixels[(y * self.width + x) as usize]
    }

    /// Clear the pixel at (x, y).
    pub fn clear_at(&mut self, x: u16, y: u16) -> &mut Self {
        if self.get(x, y).width() == 2 {
            self.get_mut(x + 1, y).set_character(' ');
        } else if self.get(x, y).is_skip() {
            self.get_mut(x - 1, y).set_character(' ');
        }
        if self.get(x, y).character().is_some() {
            self.get_mut(x, y).set_character(' ');
        }
        self
    }

    /// Directly replace the pixel at (x, y) with a new pixel.
    /// Panics if the pixel is out of bounds.
    /// Warning: This will not process two-width character and skipped pixel.
    fn replace_at(&mut self, x: u16, y: u16, pixel: Pixel) -> &mut Self {
        if x >= self.width || y >= self.height {
            panic!("Pixel out of buffer bounds: ({}, {})", x, y);
        }
        self.pixels[(y * self.width + x) as usize] = pixel;
        self
    }

    /// Render a pixel at (x, y).
    pub fn render_pixel(&mut self, x: u16, y: u16, pixel: &Pixel) -> &mut Self {
        if y < self.height && x + pixel.width() as u16 <= self.width {
            self.clear_at(x, y);
            self.get_mut(x, y).render(&pixel);
            if pixel.width() == 2 {
                self.clear_at(x + 1, y);
                self.get_mut(x + 1, y).set_style(pixel.style()).set_skip();
            }
        }
        self
    }

    pub fn render_string<T: AsRef<str>>(
        &mut self,
        string: T,
        style: Style,
        area: Area,
        wrap: bool,
    ) -> &mut Self {
        let area = Area::new(0, 0, self.width, self.height).intersect(area);
        if area.is_empty() {
            return self;
        }
        let (start_x, mut y, end_x, end_y) = area.corners();

        let mut render_line = |line: &str, mut y: u16| {
            let chars = line
                .chars()
                .filter(|char| !char.is_control())
                .map(|cell| (cell, cell.width().unwrap_or(0) as u16))
                .filter(|(_, width)| *width > 0);
            let mut x = start_x;
            for (char, width) in chars {
                if start_x + width > end_x {
                    self.render_pixel(start_x, y, Pixel::from_char(' ').set_style(style));
                    return y + 1; // All lines can not be rendered.
                }
                if x + width > end_x {
                    if end_x - x > 0 {
                        self.render_pixel(x, y, Pixel::from_char(' ').set_style(style));
                    }
                    if wrap && y < end_y {
                        x = start_x;
                        y += 1;
                    } else {
                        return y + 1;
                    }
                }
                self.render_pixel(x, y, Pixel::from_charw(char, width as u8).set_style(style));
                x += width;
            }
            y + 1
        };

        let lines = string.as_ref().lines();
        for line in lines {
            y = render_line(line, y);
            if y >= end_y {
                break;
            }
        }
        self
    }

    /// A cut of an area of buffer
    pub fn cut(&self, area: Area) -> Buffer {
        let area = Area::new(0, 0, self.width, self.height).intersect(area);
        if area.is_empty() {
            return Buffer::new(0, 0);
        }
        let mut buffer = Buffer::new(area.width, area.height);
        for y in 0..area.height {
            for x in 0..area.width {
                buffer.replace_at(x, y, self.get(area.x + x, area.y + y).clone());
            }
            if buffer.get(0, y).is_skip() {
                buffer.get_mut(0, y).set_character(' ');
            }
            if buffer.get(buffer.width - 1, y).width() == 2 {
                buffer.get_mut(buffer.width - 1, y).set_character(' ');
            }
        }
        buffer
    }

    /// Render the background of an area.
    pub fn render_background(&mut self, area: Area, background: Option<Color>) -> &mut Self {
        if background.is_none() {
            return self;
        }
        let area = Area::from_wh(self.width, self.height).intersect(area);
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                self.get_mut(x, y).set_background(background);
            }
        }
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        for pixel in self.pixels.iter_mut() {
            pixel.clear();
        }
        self
    }

    pub fn view(&self, system: ColorSystem, theme: &Theme) -> String {
        let mut out = String::new();
        let ansi = |style: &Style| format!("\x1b[0m\x1b[{}m", style.ansi_codes(system, theme));
        let mut last_style = Style::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.get(x, y);
                if pixel.style() != last_style {
                    out.push_str(&ansi(&pixel.style()));
                    last_style = pixel.style();
                }
                if let Some(character) = pixel.character() {
                    if character as u32 != 0 {
                        out.push(character);
                    }
                } else {
                    out.push(' ');
                }
            }
            out.push_str("\x1b[0m");
            last_style = Style::new();
            if y < self.height - 1 {
                out.push('\n');
            }
        }

        out
    }

    #[allow(unused_assignments)]
    pub fn render(&mut self, x: u16, y: u16, buffer: &Buffer) -> &mut Self {
        let start_x = x;
        let start_y = y;
        let width = buffer.width.min(self.width - x);
        let height = buffer.height.min(self.height - y);
        for y in start_y..start_y + height {
            self.clear_at(x, y);
            let mut x = start_x;
            while x < start_x + width {
                let pixel = buffer.get(x - start_x, y - start_y);
                if pixel.width() as u16 + x >= start_x + width {
                    break;
                }
                self.render_pixel(x, y, pixel);
                x += buffer.get(x - start_x, y - start_y).width() as u16;
            }
            if x < self.width && self.get(x, y).is_skip() {
                self.get_mut(x, y).clear_char();
            }
        }
        self
    }

    pub fn fill(&mut self, area: Area, pixel: Pixel) -> &mut Self {
        let area = Area::new(0, 0, self.width, self.height).intersect(area);
        for y in area.y..area.y + area.height {
            let mut x = area.x;
            while x < area.x + area.width {
                if x + pixel.width() as u16 >= area.x + area.width {
                    break;
                }
                self.render_pixel(x, y, &pixel);
                x += pixel.width() as u16;
            }
            if x < self.width && self.get(x, y).is_skip() {
                self.get_mut(x, y).clear_char();
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        geometry::area::Area,
        style::{color::Color, color::ColorSystem, style::Style, theme::Theme},
    };

    use super::Buffer;

    #[test]
    fn render_string() {
        let mut buffer = Buffer::new(20, 15);
        buffer
            .render_string(
                "你好，世界！\n你好",
                Style::new(),
                Area::new(0, 0, 20, 2),
                false,
            )
            .render_string("你好，世界！", Style::new(), Area::new(1, 0, 19, 15), false)
            .render_string("你好，世界！", Style::new(), Area::new(19, 0, 1, 15), false);
        for y in 0..15 {
            for x in 0..20 {
                let pixel = buffer.get(x, y);
                print!(
                    "{}",
                    if pixel.character().is_some() {
                        pixel.character().unwrap()
                    } else {
                        '-'
                    }
                );
            }
            println!();
        }
    }

    #[test]
    fn cut() {
        let mut buffer = Buffer::new(20, 15);
        buffer.render_string(
            "你好，世界！你好，世界！\n你好，世界！",
            Style::new(),
            Area::new(0, 0, 20, 15),
            false,
        );
        let buffer = buffer.cut(Area::new(1, 0, 18, 2));
        for y in 0..2 {
            for x in 0..18 {
                let pixel = buffer.get(x, y);
                print!(
                    "{}",
                    if pixel.character().is_some() {
                        pixel.character().unwrap()
                    } else {
                        '-'
                    }
                );
            }
            println!();
        }
    }

    #[test]
    fn view() {
        let mut buffer = Buffer::new(20, 15);
        let out = buffer
            .render_string(
                "你好，世界！你好，世界！\n你好，世界！",
                Style::new().foreground(Color::Text),
                Area::new(0, 0, 20, 15),
                false,
            )
            .render_background(Area::from_wh(20, 15), Some(Color::Background))
            .view(ColorSystem::TrueColor, &Theme::TOKYO_NIGHT);
        print!("{}", out);
    }
}
