use crossterm::event::Event;

use crate::{
    buffer::buffer::Buffer,
    geometry::{area::Area, length::Length, size::Size, spacing::Spacing},
    shell::Shell,
    style::{color::Color, theme::Theme},
    widget::{element::Element, widget::Widget},
};

pub struct Padding<'a, Message> {
    element: Element<'a, Message>,
    widget_bounds: Area,
    padding: Spacing,
    background: Option<Color>,
    size_hint: Size<Length>,
}

#[macro_export]
macro_rules! padding {
    ($element:expr, $padding_y:expr, $padding_x:expr) => {
        Element::new(padding::Padding::axes($element, $padding_y, $padding_x))
    };
    ($element:expr, $padding_y:expr) => {
        Element::new(padding::Padding::vertical($element, $padding_y))
    };
    ($element:expr, $padding_x:expr) => {
        Element::new(padding::Padding::horizontal($element, $padding_x))
    };
    ($element:expr, $padding_top:expr, $padding_right:expr, $padding_bottom:expr, $padding_left:expr) => {
        Element::new(padding::Padding::new(
            $element,
            $padding_top,
            $padding_right,
            $padding_bottom,
            $padding_left,
        ))
    };
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
            size_hint: Size::preferred(),
        }
    }

    pub fn axes(element: Element<'a, Message>, padding_y: u16, padding_x: u16) -> Self {
        Padding {
            element,
            widget_bounds: Area::zeros(),
            padding: Spacing::axes(padding_y, padding_x),
            background: None,
            size_hint: Size::preferred(),
        }
    }

    pub fn vertical(element: Element<'a, Message>, padding_y: u16) -> Self {
        Padding {
            element,
            widget_bounds: Area::zeros(),
            padding: Spacing::vertical(padding_y),
            background: None,
            size_hint: Size::preferred(),
        }
    }

    pub fn horizontal(element: Element<'a, Message>, padding_x: u16) -> Self {
        Padding {
            element,
            widget_bounds: Area::zeros(),
            padding: Spacing::horizontal(padding_x),
            background: None,
            size_hint: Size::preferred(),
        }
    }

    pub fn background(mut self, background: Option<Color>) -> Self {
        self.background = background;
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

impl<'a, Message> Widget<Message> for Padding<'a, Message> {
    fn layout(&mut self, viewport: Area) {
        self.widget_bounds = viewport.shrink(self.padding);
        self.element.widget_mut().layout(self.widget_bounds);
    }

    fn render(&self, area: Area, buffer: &mut Buffer, theme: &Theme) {
        buffer.render_background(area, self.background);
        self.element
            .widget()
            .render(area.shrink(self.padding), buffer, theme);
    }

    fn process_event(&mut self, event: Event, shell: &mut Shell<Message>) {
        self.element.widget_mut().process_event(event, shell)
    }

    fn size(&self) -> Size {
        self.element.widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.size_hint
    }
}
