use crossterm::event::Event;

use crate::{
    buffer::buffer::Buffer,
    geometry::{length::Length, size::Size},
    style::theme::Theme,
};

#[allow(unused_variables)]
pub trait Widget {
    fn render(&self, buffer: &mut Buffer, theme: &Theme) {}

    /// Process an event and return true if the event was alive.
    fn process_event(&mut self, event: Event) -> bool {
        true
    }

    fn size_hint(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }
}
