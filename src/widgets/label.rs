use crate::{
    buffer::buffer::Buffer,
    geometry::{area::Area, length::Length, size::Size},
    style::{color::Color, style::Style, theme::Theme},
    text::Text,
    widget::{element::Element, widget::Widget},
};

pub struct Label {
    text: Text,
    style: Style,
    wrap: bool,
    size_hint: Size<Length>,
}

pub fn label(content: impl Into<String>) -> Label {
    Label::new(content)
}

impl Label {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            text: Text::new(content),
            style: Style::new().foreground(Color::Text),
            wrap: true,
            size_hint: Size::preferred(),
        }
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.text.set(content);
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

    pub fn width(mut self, width: Length) -> Self {
        self.size_hint.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.size_hint.height = height;
        self
    }
}

impl<Message> Widget<Message> for Label {
    fn render(&self, area: Area, buffer: &mut Buffer, _theme: &Theme) {
        buffer.render_string(self.text.raw(), self.style, area, self.wrap);
    }

    fn size(&self) -> Size<u16> {
        Size::new(self.text.size().width, self.text.size().height)
    }

    fn size_hint(&self) -> Size<Length> {
        self.size_hint
    }

    #[allow(unused_variables)]
    fn layout(&mut self, viewport: Area) {}
}

impl<'a, Message> From<Label> for Element<'a, Message> {
    fn from(value: Label) -> Self {
        Self::new(value)
    }
}
