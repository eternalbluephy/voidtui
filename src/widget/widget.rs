use crossterm::event::Event;

use crate::{
    buffer::buffer::Buffer,
    geometry::{area::Area, length::Length, size::Size},
    shell::Shell,
    style::theme::Theme,
};

#[allow(unused_variables)]
pub trait Widget<Message> {
    fn render(&self, area: Area, buffer: &mut Buffer, theme: &Theme);

    fn process_event(&mut self, event: Event, shell: &mut Shell<Message>) {}

    /// Returns the preferred size of the widget.
    fn size(&self) -> Size;

    fn size_hint(&self) -> Size<Length> {
        Size::preferred()
    }

    fn layout(&mut self, viewport: Area);
}
