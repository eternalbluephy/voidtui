use unicode_width::UnicodeWidthChar;

use crate::{
    buffer::buffer::Buffer,
    geometry::{area::Area, length::Length, size::Size},
    style::{color::Color, style::Style},
    widget::{element::Element, widget::Widget},
};

pub struct Text {
    content: String,
    size: Size<u16>,
    style: Style,
    wrap: bool,
}

pub fn text<'a, T: Into<String>, Message>(content: T) -> Element<'a, Message> {
    Element::new(Text::new(content))
}

impl Text {
    pub fn new<T: Into<String>>(content: T) -> Self {
        let content = content.into();
        let size = Self::size_of(&content);
        Self {
            content,
            size,
            style: Style::new().foreground(Color::Text),
            wrap: true,
        }
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = content;
        self.size = Self::size_of(&self.content);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    fn size_of(content: &str) -> Size<u16> {
        let mut size = Size::new(0, 0);

        for line in content.lines() {
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

impl<Message> Widget<Message> for Text {
    fn render(&self, buffer: &mut Buffer) {
        buffer.render_string(
            &self.content,
            self.style,
            Area::new(0, 0, buffer.width(), buffer.height()),
            self.wrap,
        );
    }

    fn size(&self) -> Size<Length> {
        Size::new(
            Length::Fixed(self.size.width),
            Length::Fixed(self.size.height),
        )
    }

    #[allow(unused_variables)]
    fn layout(&mut self, viewport: Area) {}
}
