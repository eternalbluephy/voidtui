use crossterm::event::{MouseButton, MouseEventKind};

use crate::{
    buffer::buffer::Buffer, event::Event, geometry::{area::Area, length::Length, size::Size, spacing::Spacing}, shell::Shell, style::{color::Color, style::Style, theme::Theme}, text::Text, widget::{element::Element, widget::Widget}
};

pub struct Button<'a, Message: Clone> {
    padding: Spacing,
    content: Text,
    variant: Variant,
    on_click: Option<Message>,
    state: &'a mut State,
    size_hint: Size<Length>,
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct State {
    pub status: Status,
}

impl State {
    pub fn new() -> Self {
        Self {
            status: Status::Disabled,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Status {
    #[default]
    Disabled,
    Default,
    Hovered,
    Pressed,
}

pub fn button<Message: Clone>(state: &mut State, content: impl Into<String>) -> Button<Message> {
    Button::new(state, content)
}

impl<'a, Message: Clone> Button<'a, Message> {
    pub fn new(state: &'a mut State, content: impl Into<String>) -> Self {
        Button {
            padding: Spacing::axes(1, 2),
            content: Text::new(content),
            variant: Variant::Default,
            on_click: None,
            state,
            size_hint: Size::preferred(),
        }
    }

    pub fn on_click(mut self, message: Message) -> Self {
        self.on_click = Some(message);
        if let Status::Disabled = self.state.status {
            self.state.status = Status::Default;
        }
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
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

impl<'a, Message: Clone> Widget<Message> for Button<'a, Message> {
    fn process_event(
        &mut self,
        event: Event,
        shell: &mut Shell<Message>,
        bounds: Area
    ) {
        if let Status::Disabled = self.state.status {
            return;
        }
        if shell.is_event_captured() {
            self.state.status = Status::Default;
            return;
        }
        if let Event::Mouse(event) = event {
            if let Some(position) = event.position {
                if bounds.contains(position.x, position.y) {
                    if let MouseEventKind::Down(button) = event.kind {
                        if button == MouseButton::Left {
                            shell.capture_event();
                            self.state.status = Status::Pressed;
                        }
                    } else if let MouseEventKind::Up(button) = event.kind {
                        if button == MouseButton::Left {
                            if let Status::Pressed = self.state.status {
                                shell.capture_event();
                                shell.try_push(self.on_click.clone());
                            }
                            self.state.status = Status::Hovered;
                        }
                    } else {
                        self.state.status = Status::Hovered;
                    }
                } else {
                    self.state.status = Status::Default;
                }
            } else {
                self.state.status = Status::Default;
            }
        }
    }

    fn render(&self, area: Area, buffer: &mut Buffer, theme: &Theme) {
        let style = self.variant.style();
        let background = {
            if let Status::Hovered = self.state.status {
                let background = style.background.on_theme(theme);
                Color::RGB(background.brighter(0.1))
            } else {
                style.background
            }
        };
        buffer
            .render_background(area, Some(background))
            .render_string(
                self.content.raw(),
                Style::new().foreground(style.foreground),
                area.shrink(self.padding),
                true,
            );
        if !area.is_empty() {
            let background = background.on_theme(theme);
            let highlight = background.brighter(0.2);
            let shadow = background.darker(0.2);
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

impl<'a, Message: Clone + 'a> From<Button<'a, Message>> for Element<'a, Message> {
    fn from(button: Button<'a, Message>) -> Self {
        Self::new(button)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonStyle {
    pub foreground: Color,
    pub background: Color,
}

impl ButtonStyle {
    pub fn default() -> Self {
        Self {
            foreground: Color::Text,
            background: Color::Background,
        }
    }

    pub fn primary() -> Self {
        Self {
            foreground: Color::Background,
            background: Color::Primary,
        }
    }

    pub fn success() -> Self {
        Self {
            foreground: Color::Background,
            background: Color::Success,
        }
    }

    pub fn warning() -> Self {
        Self {
            foreground: Color::Background,
            background: Color::Warning,
        }
    }

    pub fn danger() -> Self {
        Self {
            foreground: Color::Background,
            background: Color::Danger,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Default,
    Primary,
    Success,
    Warning,
    Danger,
}

impl Variant {
    pub fn style(&self) -> ButtonStyle {
        match self {
            Variant::Default => ButtonStyle::default(),
            Variant::Primary => ButtonStyle::primary(),
            Variant::Success => ButtonStyle::success(),
            Variant::Warning => ButtonStyle::warning(),
            Variant::Danger => ButtonStyle::danger(),
        }
    }
}
