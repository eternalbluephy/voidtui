use crossterm::event::Event;

use crate::{
    buffer::buffer::Buffer,
    style::{color::Color, color::ColorSystem, theme::Theme},
};

use super::widget::Widget;

pub struct Screen {
    widget: Option<Box<dyn Widget>>,
    buffer: Buffer,
    background: Color,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            widget: None,
            buffer: Buffer::new(0, 0),
            background: Color::Background,
        }
    }

    pub fn with_widget(widget: Box<dyn Widget>) -> Self {
        let mut screen = Self {
            widget: Some(widget),
            buffer: Buffer::new(0, 0),
            background: Color::Background,
        };
        screen.buffer.fill_background(screen.background);
        screen
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.buffer = Buffer::new(width, height);
    }

    pub fn width(&self) -> u16 {
        self.buffer.width()
    }

    pub fn height(&self) -> u16 {
        self.buffer.height()
    }

    pub fn view(&self, system: ColorSystem, theme: &Theme) -> String {
        if let Some(widget) = &self.widget {
            let mut buffer = self.buffer.clone();
            widget.render(&mut buffer, theme);
        }
        self.buffer.view(system, theme)
    }
}

impl Widget for Screen {
    fn process_event(&mut self, event: Event) -> bool {
        if let Some(widget) = &mut self.widget {
            return widget.process_event(event.clone());
        }
        true
    }
}
