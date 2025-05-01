use std::io::Result;

use crossterm::event::{KeyCode, MouseEvent};
use voidtui::{
    app::{App, Program}, event::Event, geometry::point::Point, shell::Shell, style::color::Color, vertical, widget::element::Element, widgets::{
        button::{self, button, Button}, label::label, scrollable::{self, Scrollable}, vertical::{HorizontalAlignment, Vertical}, viewport::{viewport, Viewport}
    }
};

#[derive(Clone)]
pub enum Message {
    Increase,
    Decrease,
}

struct Counter {
    count: i64,
    button_states: Vec<button::State>,
    scrollable: scrollable::State,
    should_stop: bool
}

impl Counter {
    fn new() -> Self {
        Self {
            count: 0,
            button_states: vec![button::State::new(); 10],
            scrollable: scrollable::State::default(),
            should_stop: false,
        }
    }
}

impl Program<Message> for Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increase => self.count += 1,
            Message::Decrease => self.count -= 1,
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut vertical = vertical![];
        for (i, state) in self.button_states.iter_mut().enumerate() {
            vertical.push(button(
                state,
                format!("Button {}", i)
            ).on_click(Message::Increase));
        }
        Scrollable::new(
            &mut self.scrollable,
            vertical.into()
        )
            .into()
    }

    fn process_event(&mut self, event: Event, _shell: &mut Shell<Message>) {
        if let Event::Key(e) = event {
            if e.code == KeyCode::Char('q') {
                self.should_stop = true;
            }
        }
    }

    fn should_stop(&self) -> bool {
        self.should_stop
    }
}

fn main() -> Result<()> {
    App::new().run(Counter::new())?;
    Ok(())
}
