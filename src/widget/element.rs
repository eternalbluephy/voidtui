use super::widget::Widget;

pub struct Element<'a, Message> {
    widget: Box<dyn Widget<Message> + 'a>,
}

impl<'a, Message> Element<'a, Message> {
    pub fn new(widget: impl Widget<Message> + 'a) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }

    pub fn widget(&self) -> &dyn Widget<Message> {
        self.widget.as_ref()
    }

    pub fn widget_mut(&mut self) -> &mut dyn Widget<Message> {
        self.widget.as_mut()
    }
}
