use voidtui::{
    widget::{
        app::{App, Program},
        element::Element,
    },
    widgets::text,
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
        text::text(self.count.to_string())
    }
}

fn main() {
    App::new(Counter::default()).run();
}
