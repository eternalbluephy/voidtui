/// An event shell.
#[derive(Debug)]
pub struct Shell<Message> {
    messages: Vec<Message>,
    event_captured: bool,
}

impl<Message> Shell<Message> {
    pub const fn new() -> Self {
        Self {
            messages: Vec::new(),
            event_captured: false,
        }
    }

    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn is_event_captured(&self) -> bool {
        self.event_captured
    }

    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }
}
