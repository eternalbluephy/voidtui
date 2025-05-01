use crate::{
    buffer::buffer::Buffer, event::Event, geometry::{area::Area, length::Length, point::Point, size::Size}, shell::Shell, style::theme::Theme, widget::{element::Element, widget::Widget}
};

pub struct Viewport<'a, Message> {
    element: Element<'a, Message>,
    size_hint: Size<Length>,
    offset: Point,
}

pub fn viewport<'a, Message>(element: Element<'a, Message>) -> Viewport<'a, Message> {
    Viewport::new(element)
}

impl<'a, Message> Viewport<'a, Message> {
    pub fn new(element: Element<'a, Message>) -> Self {
        Self {
            element,
            size_hint: Size::preferred(),
            offset: Point::new(0, 0)
        }
    }

    pub fn x(mut self, x: u16) -> Self {
        self.offset.x = x;
        self
    }

    pub fn y(mut self, y: u16) -> Self {
        self.offset.y = y;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.size_hint.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.size_hint.height = height.into();
        self
    }

    pub fn set_x(&mut self, x: u16) -> &mut Self {
        self.offset.x = x;
        self
    }

    pub fn set_y(&mut self, y: u16) -> &mut Self {
        self.offset.y = y;
        self
    }
}

impl<'a, Message> Widget<Message> for Viewport<'a, Message> {
    fn layout(&mut self, viewport: Area) {
        let size = self.element.widget().size();
        self.element.widget_mut().layout(Area::new(viewport.x, viewport.y, size.width, size.height));
    }

    fn process_event(
        &mut self,
        event: Event,
        shell: &mut Shell<Message>,
        bounds: Area,
    ) {
        match event {
            Event::Mouse(mouse_event) => {
                let position = match mouse_event.position {
                    Some(position) => {
                        if bounds.contains(position.x, position.y) {
                            Some(Point::new(position.x + self.offset.x, position.y + self.offset.y))
                        } else {
                            None
                        }
                    },
                    None => None,
                };
                self.element.widget_mut().process_event(
                    Event::Mouse(mouse_event.position(position)),
                    shell, bounds
                );
            },
            _ => self.element.widget_mut().process_event(event, shell, bounds),
        }
    }

    fn render(&self, area: Area, buffer: &mut Buffer, theme: &Theme) {
        let width = if let Length::Fixed(width) = self.element.widget().size_hint().width {
            width
        } else {
            self.element.widget().size().width
        };
        let height = if let Length::Fixed(height) = self.element.widget().size_hint().height {
            height
        } else {
            self.element.widget().size().height
        };
        let mut whole_buffer = Buffer::new(width, height);
        self.element
            .widget()
            .render(Area::new(0, 0, width, height), &mut whole_buffer, theme);
        let cut_buffer = whole_buffer.cut(Area::new(
            self.offset.x,
            self.offset.y,
            area.width,
            area.height,
        ));
        buffer.render(area.x, area.y, &cut_buffer);
    }

    fn size(&self) -> Size {
        self.element.widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.size_hint
    }

    fn update(&mut self) {
        self.element.widget_mut().update();
    }
}

impl<'a, Message: 'a> From<Viewport<'a, Message>> for Element<'a, Message> {
    fn from(value: Viewport<'a, Message>) -> Self {
        Self::new(value)
    }
}