use crate::{
    buffer::buffer::Buffer,
    geometry::{area::Area, length::Length, size::Size},
    shell::Shell,
    style::{color::Color, theme::Theme},
    widget::{element::Element, widget::Widget},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalAlignment {
    Start,
    Center,
    End,
}

pub struct Vertical<'a, Message> {
    children: Vec<Element<'a, Message>>,
    alignment: HorizontalAlignment,
    background: Option<Color>,
    children_bounds: Vec<Area>,
    preferred_size: Size,
    size_hint: Size<Length>,
    spacing: u16,
}

#[macro_export]
macro_rules! vertical {
    () => {
        Vertical::new()
    };
    ($($element:expr), + $(,)?) => {
        Vertical::with_children(vec![$($element),+])
    };
}

impl<'a, Message> Vertical<'a, Message> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            alignment: HorizontalAlignment::Start,
            background: None,
            children_bounds: Vec::new(),
            preferred_size: Size::new(0, 0),
            size_hint: Size::preferred(),
            spacing: 0,
        }
    }
    pub fn with_children(children: Vec<Element<'a, Message>>) -> Self {
        let mut children_bounds = Vec::with_capacity(children.len());
        let mut preferred_size = Size::new(0, 0);
        for child in children.iter() {
            children_bounds.push(Area::zeros());
            preferred_size.width = preferred_size.width.max(child.widget().size().width);
            preferred_size.height += child.widget().size().height;
        }
        Self {
            children,
            alignment: HorizontalAlignment::Start,
            background: None,
            children_bounds,
            preferred_size,
            size_hint: Size::preferred(),
            spacing: 0,
        }
    }

    pub fn alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn background(mut self, background: Option<Color>) -> Self {
        self.background = background;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.size_hint.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.size_hint.height = height.into();
        self
    }

    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }
}

impl<'a, Message> Widget<Message> for Vertical<'a, Message> {
    fn layout(&mut self, viewport: Area) {
        let children_widths: Vec<_> = self
            .children
            .iter()
            .map(|child| match child.widget().size_hint().width {
                Length::Preferred => child.widget().size().width.min(viewport.width),
                Length::Fixed(width) => width.min(viewport.width),
                _ => viewport.width,
            })
            .collect();
        let children_heights = Length::resolve(
            viewport.height,
            self.children
                .iter()
                .map(|child| child.widget().size_hint().height)
                .collect(),
            self.children
                .iter()
                .map(|child| child.widget().size().height)
                .collect(),
        );

        let mut y = viewport.y;
        for (i, (width, height)) in children_widths
            .into_iter()
            .zip(children_heights.into_iter())
            .enumerate()
        {
            let x = match self.alignment {
                HorizontalAlignment::Start => viewport.x,
                HorizontalAlignment::Center => {
                    viewport
                        .x
                        .saturating_add(viewport.width.saturating_sub(width))
                        / 2
                }
                HorizontalAlignment::End => viewport
                    .x
                    .saturating_add(viewport.width)
                    .saturating_sub(width),
            };
            let bounds = Area::new(
                x,
                y,
                width,
                height.min((viewport.y + viewport.height).saturating_sub(y)),
            );
            self.children_bounds[i] = bounds;
            self.children[i].widget_mut().layout(bounds);
            y += height + self.spacing;
        }
    }

    fn process_event(&mut self, event: crossterm::event::Event, shell: &mut Shell<Message>) {
        if shell.is_event_captured() {
            return;
        }

        for child in &mut self.children.iter_mut().rev() {
            child.widget_mut().process_event(event.clone(), shell);
        }
    }

    fn render(&self, area: Area, buffer: &mut Buffer, theme: &Theme) {
        buffer.render_background(area, self.background);
        for (child, bounds) in self.children.iter().zip(self.children_bounds.iter()) {
            child.widget().render(*bounds, buffer, theme);
        }
    }

    fn size(&self) -> Size {
        Size::new(
            self.preferred_size.width,
            self.preferred_size.height + self.spacing * (self.children.len() as u16 - 1),
        )
    }

    fn size_hint(&self) -> Size<Length> {
        self.size_hint
    }
}

impl<'a, Message: 'a> From<Vertical<'a, Message>> for Element<'a, Message> {
    fn from(value: Vertical<'a, Message>) -> Self {
        Self::new(value)
    }
}
