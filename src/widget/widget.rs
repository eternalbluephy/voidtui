use crossterm::event::Event;

use crate::{
    buffer::buffer::Buffer,
    geometry::{area::Area, length::Length, size::Size},
    shell::Shell,
};

#[allow(unused_variables)]
pub trait Widget<Message> {
    fn render(&self, buffer: &mut Buffer);

    fn process_event(&mut self, event: Event, shell: &mut Shell<Message>) {}

    fn size(&self) -> Size<Length>;

    fn layout(&mut self, viewport: Area);
}
