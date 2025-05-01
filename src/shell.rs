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

    /// Push the message to shell.
    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    /// Push the optional message to shell.
    /// 
    /// Equals to
    /// ```no_run
    /// if let Some(message) = message {
    ///     self.push(message);
    /// }
    /// ```
    pub fn try_push(&mut self, message: Option<Message>) {
        if let Some(message) = message {
            self.push(message);
        }
    }

    pub fn capture_event(&mut self) {
        self.event_captured = true;
    }

    pub fn is_event_captured(&self) -> bool {
        self.event_captured
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }
}
