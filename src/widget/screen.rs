use crossterm::event::Event;

use crate::{
    buffer::buffer::Buffer,
    geometry::{area::Area, length::Length, size::Size},
    shell::Shell,
    style::{
        color::{Color, ColorSystem},
        theme::Theme,
    },
    terminal,
};

use super::{element::Element, widget::Widget};

pub struct Screen<'a, Message> {
    element: Option<Element<'a, Message>>,
    widget_bounds: Area,
    buffer: Buffer,
    background: Option<Color>,
}

impl<'a, Message> Screen<'a, Message> {
    pub fn new() -> Self {
        let (width, height) = terminal::size().into();
        Self {
            element: None,
            widget_bounds: Area::zeros(),
            buffer: Buffer::new(width, height),
            background: None,
        }
    }

    pub fn with_element(element: Element<'a, Message>) -> Self {
        let (width, height) = terminal::size().into();
        let mut screen = Self {
            element: Some(element),
            widget_bounds: Area::zeros(),
            buffer: Buffer::new(width, height),
            background: None,
        };
        screen.buffer.fill_background(screen.background);
        screen
    }

    pub fn set_element(&mut self, element: Element<'a, Message>) {
        self.element = Some(element);
    }

    pub fn drop_element(&mut self) {
        self.element = None;
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.buffer = Buffer::new(width, height);
        self.buffer.fill_background(self.background);
    }

    pub fn width(&self) -> u16 {
        self.buffer.width()
    }

    pub fn height(&self) -> u16 {
        self.buffer.height()
    }

    pub fn background(&self) -> Option<Color> {
        self.background
    }

    pub fn set_background(&mut self, color: Option<Color>) {
        self.background = color;
        self.buffer.fill_background(color);
    }

    pub fn draw(&mut self, system: ColorSystem, theme: &Theme) -> String {
        if let Some(element) = &self.element {
            let mut buffer = Buffer::new(self.widget_bounds.width, self.widget_bounds.height);
            element.widget().render(&mut buffer);
            self.buffer.render(0, 0, &buffer);
        }
        self.buffer.view(system, theme)
    }
}

impl<'a, Message> Widget<Message> for Screen<'a, Message> {
    #[allow(unused_variables)]
    fn render(&self, buffer: &mut Buffer) {}

    fn process_event(&mut self, event: Event, shell: &mut Shell<Message>) {
        if shell.is_event_captured() {
            return;
        }
        if let Some(element) = &mut self.element {
            element.widget_mut().process_event(event.clone(), shell);
        }
    }

    /// Size for screen is unnecessary.
    /// The size would always be the terminal size.
    /// You can return any size.
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(&mut self, viewport: Area) {
        if let Some(element) = &mut self.element {
            let width = match element.widget_mut().size().width {
                Length::Fixed(width) => width,
                _ => viewport.width,
            };
            let height = match element.widget_mut().size().height {
                Length::Fixed(height) => height,
                _ => viewport.height,
            };
            element.widget_mut().layout(Area::new(0, 0, width, height));
            self.widget_bounds = Area::new(0, 0, width, height);
        }
    }
}
