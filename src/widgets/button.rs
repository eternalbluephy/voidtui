use crossterm::event::{Event, MouseButton, MouseEvent, MouseEventKind};

use crate::{
    buffer::buffer::Buffer,
    geometry::{area::Area, length::Length, size::Size, spacing::Spacing},
    shell::Shell,
    style::color::Color,
    widget::widget::Widget,
};

pub struct Button<Message: Clone> {
    padding: Spacing,
    content: String,
    content_color: Color,
    background: Color,
    bounds: Area,
    on_click: Option<Message>,
}

impl<Message: Clone> Button<Message> {
    pub fn new(content: impl Into<String>) -> Self {
        Button {
            padding: Spacing::axes(1, 2),
            content: content.into(),
            content_color: Color::Text,
            background: Color::Primary,
            bounds: Area::zeros(),
            on_click: None,
        }
    }

    pub fn on_click(mut self, message: Message) -> Self {
        self.on_click = Some(message);
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    pub fn padding(mut self, padding: impl Into<Spacing>) -> Self {
        self.padding = padding.into();
        self
    }
}

impl<Message: Clone> Widget<Message> for Button<Message> {
    fn layout(&mut self, viewport: Area) {
        self.bounds = viewport;
    }

    fn process_event(&mut self, event: Event, shell: &mut Shell<Message>) {
        if shell.is_event_captured() {
            return;
        }
        if let Event::Mouse(event) = event {
            if self.bounds.contains(event.column, event.row) {
                if let MouseEventKind::Down(button) = event.kind {
                    if button == MouseButton::Left {}
                }
            }
        }
    }

    fn render(&self, buffer: &mut Buffer) {}

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(0), Length::Fixed(0))
    }
}
