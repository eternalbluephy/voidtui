use crossterm::event::{Event, MouseButton, MouseEventKind};

use crate::{
    buffer::buffer::Buffer,
    geometry::{area::Area, length::Length, size::Size, spacing::Spacing},
    shell::Shell,
    style::{color::Color, style::Style, theme::Theme},
    text::Text,
    widget::{element::Element, widget::Widget},
};

pub struct Button<Message: Clone> {
    padding: Spacing,
    content: Text,
    content_style: Style,
    background: Option<Color>,
    bounds: Area,
    on_click: Option<Message>,
    hovering: bool,
    size_hint: Size<Length>,
}

pub fn button<'a, Message: 'a + Clone>(content: impl Into<String>) -> Button<Message> {
    Button::new(content)
}

impl<Message: Clone> Button<Message> {
    pub fn new(content: impl Into<String>) -> Self {
        Button {
            padding: Spacing::axes(1, 2),
            content: Text::new(content),
            content_style: Style::new().foreground(Color::Text),
            background: Some(Color::Background),
            bounds: Area::zeros(),
            on_click: None,
            hovering: false,
            size_hint: Size::preferred(),
        }
    }

    pub fn on_click(mut self, message: Message) -> Self {
        self.on_click = Some(message);
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content.set(content);
        self
    }

    pub fn padding(mut self, padding: impl Into<Spacing>) -> Self {
        self.padding = padding.into();
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
                self.hovering = true;
                if let MouseEventKind::Down(button) = event.kind {
                    if button == MouseButton::Left {
                        if let Some(message) = self.on_click.clone() {
                            shell.push(message);
                        }
                    }
                }
            }
        }
    }

    fn render(&self, area: Area, buffer: &mut Buffer, theme: &Theme) {
        buffer
            .render_background(area, self.background)
            .render_string(
                self.content.raw(),
                self.content_style,
                area.shrink(self.padding),
                true,
            );
        if !area.is_empty() {
            if let Some(background) = self.background {
                let highlight = background.on_theme(theme).brighter(0.2);
                let shadow = background.on_theme(theme).darker(0.2);
                buffer
                    .render_string(
                        "▁".repeat(area.width as usize),
                        Style::new().foreground(shadow),
                        Area::new(area.x, area.y + area.height - 1, area.width, 1),
                        false,
                    )
                    .render_string(
                        "▔".repeat(area.width as usize),
                        Style::new().foreground(highlight),
                        Area::new(area.x, area.y, area.width, 1),
                        false,
                    );
            }
        }
    }

    fn size(&self) -> Size {
        Size::new(
            self.content.size().width + self.padding.left + self.padding.right,
            self.content.size().height + self.padding.top + self.padding.bottom,
        )
    }

    fn size_hint(&self) -> Size<Length> {
        self.size_hint
    }
}

impl<'a, Message: Clone + 'a> From<Button<Message>> for Element<'a, Message> {
    fn from(button: Button<Message>) -> Self {
        Self::new(button)
    }
}
