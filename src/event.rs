use crossterm::event::{KeyEvent, KeyModifiers, MouseEventKind};

use crate::geometry::{point::Point, size::Size};

#[derive(Debug, Clone, Copy, Hash)]
pub enum Event {
    Unknown,
    FocusGained,
    FocusLost,
    Key(KeyEvent),
    Mouse(MouseEvent),
    WindowResize(Size),
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct MouseEvent {
    pub kind: MouseEventKind,
    /// The absolute position of the mouse in the terminal.
    pub absolute_position: Point,
    /// The position that may be translated by container.
    /// None for mouse events that are not in container.
    pub position: Option<Point>,
    pub modifiers: KeyModifiers,
}

impl MouseEvent {
    pub fn position(&self, position: Option<Point>) -> Self {
        Self {
            position,
            ..*self
        }
    }
}

impl From<crossterm::event::Event> for Event {
    fn from(value: crossterm::event::Event) -> Self {
        match value {
            crossterm::event::Event::FocusGained => Event::FocusGained,
            crossterm::event::Event::FocusLost => Event::FocusLost,
            crossterm::event::Event::Key(event) => Event::Key(event),
            crossterm::event::Event::Mouse(event) => {
                let mouse_event = MouseEvent {
                    kind: event.kind,
                    absolute_position: Point::new(event.column, event.row),
                    position: Some(Point::new(event.column, event.row)),
                    modifiers: event.modifiers
                };
                Event::Mouse(mouse_event)
            },
            crossterm::event::Event::Resize(width, height) => Event::WindowResize(Size::new(width, height)),
            _ => Event::Unknown,
        }
    }
}