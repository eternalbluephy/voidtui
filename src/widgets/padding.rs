use crossterm::event::Event;

use crate::{
    buffer::buffer::Buffer,
    geometry::{area::Area, length::Length, size::Size, spacing::Spacing},
    shell::Shell,
    style::color::Color,
    widget::{element::Element, widget::Widget},
};

pub struct Padding<'a, Message> {
    element: Element<'a, Message>,
    widget_bounds: Area,
    padding: Spacing,
    background: Option<Color>,
}

impl<'a, Message> Padding<'a, Message> {
    pub fn new(
        element: Element<'a, Message>,
        padding_top: u16,
        padding_right: u16,
        padding_bottom: u16,
        padding_left: u16,
    ) -> Self {
        Padding {
            element,
            widget_bounds: Area::zeros(),
            padding: Spacing::new(padding_top, padding_right, padding_bottom, padding_left),
            background: None,
        }
    }

    pub fn axes(element: Element<'a, Message>, padding_y: u16, padding_x: u16) -> Self {
        Padding {
            element,
            widget_bounds: Area::zeros(),
            padding: Spacing::axes(padding_y, padding_x),
            background: None,
        }
    }

    pub fn vertical(element: Element<'a, Message>, padding_y: u16) -> Self {
        Padding {
            element,
            widget_bounds: Area::zeros(),
            padding: Spacing::vertical(padding_y),
            background: None,
        }
    }

    pub fn horizontal(element: Element<'a, Message>, padding_x: u16) -> Self {
        Padding {
            element,
            widget_bounds: Area::zeros(),
            padding: Spacing::horizontal(padding_x),
            background: None,
        }
    }

    pub fn background(mut self, background: Option<Color>) -> Self {
        self.background = background;
        self
    }
}

impl<'a, Message> Widget<Message> for Padding<'a, Message> {
    fn layout(&mut self, viewport: Area) {
        self.widget_bounds = viewport.shrink(self.padding);
        self.element.widget_mut().layout(self.widget_bounds);
    }

    fn render(&self, buffer: &mut Buffer) {
        buffer.fill_background(self.background);
        let mut widget_buffer = Buffer::new(self.widget_bounds.width, self.widget_bounds.height);
        self.element.widget().render(&mut widget_buffer);
        buffer.render(self.widget_bounds.x, self.widget_bounds.y, &widget_buffer);
    }

    fn process_event(&mut self, event: Event, shell: &mut Shell<Message>) {
        self.element.widget_mut().process_event(event, shell)
    }

    fn size(&self) -> Size<Length> {
        self.element.widget().size()
    }
}
