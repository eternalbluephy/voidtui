use voidtui::{
    vertical,
    widget::{
        app::{App, Program},
        element::Element,
    },
    widgets::{
        button::button,
        label::label,
        vertical::{Vertical, VerticalAlignment},
    },
};

#[derive(Clone)]
pub enum Message {
    Increase,
    Decrease,
}

#[derive(Default)]
struct Counter {
    count: i64,
}

impl<'a> Program<'a, Message> for Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increase => self.count += 1,
            Message::Decrease => self.count -= 1,
        }
    }

    fn view(&self) -> Element<'a, Message> {
        vertical![
            button("Increase").on_click(Message::Increase).into(),
            label(self.count.to_string()).into(),
            button("Decrease").on_click(Message::Decrease).into(),
        ]
        .alignment(VerticalAlignment::Center)
        .into()
    }
}

fn main() {
    App::new(Counter::default()).run();
}
