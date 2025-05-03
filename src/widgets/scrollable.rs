use crossterm::event::MouseEventKind;

use crate::{
    buffer::{buffer::Buffer, pixel::Pixel},
    event::Event,
    geometry::{area::Area, length::Length, point::Point, size::Size},
    shell::Shell,
    style::{color::Color, style::Style, theme::Theme},
    widget::{element::Element, widget::Widget},
};

pub enum ScrollBarVisibility {
    Auto,
    Always,
    Nerver,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Status {
    Default,
    Hovered,
    Pressed,
}

impl Default for Status {
    fn default() -> Self {
        Status::Default
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct State {
    pub offset: Point,
    pub status: Status,
    pub mouse_down_position: Option<Point>,
}

pub struct Scrollable<'a, Message> {
    element: Element<'a, Message>,
    vertical: ScrollBarVisibility,
    horizontal: ScrollBarVisibility,
    size_hint: Size<Length>,
    state: &'a mut State,
}

pub fn scrollable<'a, Message>(state: &'a mut State, element: Element<'a, Message>) -> Scrollable<'a, Message> {
    Scrollable::new(state, element)
}

impl<'a, Message> Scrollable<'a, Message> {
    pub fn new(state: &'a mut State, element: Element<'a, Message>) -> Self {
        Self {
            element,
            vertical: ScrollBarVisibility::Auto,
            horizontal: ScrollBarVisibility::Auto,
            size_hint: Size::preferred(),
            state,
        }
    }

    pub fn vertical(mut self, visible: ScrollBarVisibility) -> Self {
        self.vertical = visible;
        self
    }

    pub fn horizontal(mut self, visible: ScrollBarVisibility) -> Self {
        self.horizontal = visible;
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

    fn should_vertical_scroll(&self, height: u16) -> bool {
        match self.vertical {
            ScrollBarVisibility::Always => true,
            ScrollBarVisibility::Nerver => false,
            ScrollBarVisibility::Auto => match self.element.widget().size_hint().height {
                Length::Preferred => self.element.widget().size().height > height,
                Length::Fixed(widget_height) => widget_height > height,
                _ => false,
            },
        }
    }

    fn should_horizontal_scroll(&self, width: u16) -> bool {
        match self.horizontal {
            ScrollBarVisibility::Always => true,
            ScrollBarVisibility::Nerver => false,
            ScrollBarVisibility::Auto => match self.element.widget().size_hint().width {
                Length::Preferred => self.element.widget().size().width > width,
                Length::Fixed(widget_width) => widget_width > width,
                _ => false,
            },
        }
    }

    fn vertical_bar_bounds(&self, bounds: Area) -> Area {
        if self.should_vertical_scroll(bounds.height) {
            Area::new(bounds.x + bounds.width - 1, bounds.y, 1, bounds.height)
        } else {
            Area::zeros()
        }
    }

    fn vertical_draggable_bounds(&self, bounds: Area, content_height: u16) -> Area {
        if self.should_vertical_scroll(bounds.height) {
            let bar_height = ((bounds.height as f32 * (bounds.height as f32 / content_height as f32)) as u16)
                .max(1);
            let max_scroll = content_height.saturating_sub(bounds.height);
            let scroll_ratio = if max_scroll > 0 {
                self.state.offset.y as f32 / max_scroll as f32
            } else {
                0.0
            };
            let available_space = bounds.height.saturating_sub(bar_height);
            let bar_y = (scroll_ratio * available_space as f32).round() as u16;
            
            Area::new(bounds.x + bounds.width - 1, bounds.y + bar_y, 1, bar_height)
        } else {
            Area::zeros()
        }
    }

    fn horizontal_bar_bounds(&self, bounds: Area) -> Area {
        if self.should_horizontal_scroll(bounds.width) {
            Area::new(
                bounds.x,
                bounds.y + bounds.height - 1,
                bounds.width
                    .saturating_sub(if self.should_vertical_scroll(bounds.height) {
                        1
                    } else {
                        0
                    }),
                1,
            )
        } else {
            Area::zeros()
        }
    }

    fn horizontal_draggable_bounds(&self, bounds: Area, content_width: u16) -> Area {
        if self.should_horizontal_scroll(bounds.width) {
            let bar_width = ((bounds.width as f32 * (bounds.width as f32 / content_width as f32))
                as u16)
                .max(1);
            let bar_mid_x =
                ((bounds.width as f32 * (self.state.offset.x as f32 / content_width as f32))
                    as u16)
                    .min(self.horizontal_bar_bounds(bounds).width.saturating_sub(bar_width));
            let bar_x = bar_mid_x.saturating_sub(bar_width / 2);
            Area::new(bar_x, bounds.y + bounds.height - 1, bar_width, 1)
        } else {
            Area::zeros()
        }
    }

    fn vertical_draggable_to_offset(
        bar_bounds: Area,
        draggable_bounds: Area,
        element_height: u16,
    ) -> u16 {
        if draggable_bounds.y == bar_bounds.y {
            return 0;
        } else if draggable_bounds.y + draggable_bounds.height >= bar_bounds.height {
            return element_height.saturating_sub(bar_bounds.height);
        }

        let scroll_ratio = (draggable_bounds.y as f32 - bar_bounds.y as f32)
            / (bar_bounds.height as f32 - draggable_bounds.height as f32);
        let element_y = scroll_ratio * (element_height as f32 - bar_bounds.height as f32);

        element_y
            .min(element_height as f32 - bar_bounds.height as f32)
            .max(0.0)
            .round() as u16
    }
}

impl<'a, Message> Widget<Message> for Scrollable<'a, Message> {
    fn layout(&mut self, viewport: Area) {
        let size = self.element.widget().size();
        self.element.widget_mut().layout(Area::new(
            viewport.x,
            viewport.y,
            size.width,
            size.height,
        ));
    }

    fn process_event(&mut self, event: Event, shell: &mut Shell<Message>, bounds: Area) {
        let vertical_bar_bounds = self.vertical_bar_bounds(bounds);
        let vertical_draggable_bounds = self.vertical_draggable_bounds(bounds, self.element.widget().size().height);
        let horizontal_bar_bounds = self.horizontal_bar_bounds(bounds);
        let horizontal_draggable_bounds = self.horizontal_draggable_bounds(bounds, self.element.widget().size().width);
        let content_bounds = Area::new(
            bounds.x,
            bounds.y,
            bounds.width
                - if self.should_vertical_scroll(bounds.height) {
                    1
                } else {
                    0
                },
            bounds.height
                - if self.should_horizontal_scroll(bounds.width) {
                    1
                } else {
                    0
                },
        );
        let element_width = if let Length::Fixed(width) = self.element.widget().size_hint().width {
            width
        } else {
            self.element.widget().size().width
        };
        let element_height = if let Length::Fixed(height) = self.element.widget().size_hint().height
        {
            height
        } else {
            self.element.widget().size().height
        };

        // Process mouse events for content.
        if let Event::Mouse(event) = event {
            if let Some(position) = event.position {
                if content_bounds.contains(position.x, position.y) {
                    // Just offset the mouse position.
                    self.element.widget_mut().process_event(
                        Event::Mouse(event.position(Some(Point::new(
                            position.x + self.state.offset.x,
                            position.y + self.state.offset.y,
                        )))),
                        shell,
                        Area::new(bounds.x, bounds.y, element_width, element_height)
                    );
                } else {
                    self.element.widget_mut().process_event(
                        Event::Mouse(event.position(None)),
                        shell,
                        Area::new(bounds.x, bounds.y, element_width, element_height),
                    );
                }
            } else {
                self.element.widget_mut().process_event(
                    Event::Mouse(event.position(None)),
                    shell,
                    Area::new(bounds.x, bounds.y, element_width, element_height),
                );
            }
        }

        // Process events for scrollbar.
        if shell.is_event_captured() || bounds.is_empty() {
            return;
        }
        match event {
            Event::Mouse(event) => {
                if let Some(position) = event.position {
                    match event.kind {
                        MouseEventKind::Down(_) => {
                            // Jump to the position.
                            if vertical_bar_bounds.contains(position.x, position.y)
                                && !vertical_draggable_bounds.contains(position.x, position.y)  {
                                shell.capture_event();
                                let expected_draggable_y = (position.y as i32 - vertical_draggable_bounds.y as i32 / 2)
                                    .min(bounds.height as i32 - vertical_draggable_bounds.height as i32)
                                    .max(0) as u16;
                                self.state.offset.y = Self::vertical_draggable_to_offset(vertical_bar_bounds, vertical_draggable_bounds.y(expected_draggable_y), element_height);
                            } else if horizontal_bar_bounds.contains(position.x, position.y)
                                && !horizontal_draggable_bounds.contains(position.x, position.y) {
                                shell.capture_event();
                                // TODO: horizontal draggable.

                            } else if vertical_draggable_bounds.contains(position.x, position.y)
                                || horizontal_draggable_bounds.contains(position.x, position.y) {
                                shell.capture_event();
                                self.state.mouse_down_position = Some(position)
                            }
                        },
                        MouseEventKind::Drag(_) => {
                            if let Some(down_position) = self.state.mouse_down_position {
                                shell.capture_event();
                                if vertical_draggable_bounds.contains(down_position.x, down_position.y) {
                                    let expected_draggable_y = (vertical_draggable_bounds.y as i32 + position.y as i32 - down_position.y as i32)
                                        .min(bounds.height as i32 - vertical_draggable_bounds.height as i32)
                                        .max(0) as u16;
                                    self.state.offset.y = Self::vertical_draggable_to_offset(vertical_bar_bounds, vertical_draggable_bounds.y(expected_draggable_y), element_height);
                                    self.state.mouse_down_position = Some(position);
                                }
                                // TODO: horizontal draggable.
                            }
                        },
                        MouseEventKind::ScrollUp => {
                            shell.capture_event();
                            self.state.offset.y = self.state.offset.y.saturating_sub(1);
                        },
                        MouseEventKind::ScrollDown => {
                            shell.capture_event();
                            self.state.offset.y = self.state.offset.y
                                .saturating_add(1)
                                .min(element_height.saturating_sub(bounds.height));
                        }
                        _ => (),
                    }
                } else {
                    self.state.mouse_down_position = None;
                }
            }
            _ => (),
        }
    }

    fn update(&mut self) {
        self.element.widget_mut().update();
    }

    fn render(&self, area: Area, buffer: &mut Buffer, theme: &Theme) {
        // Render the whole content then cut it.
        let content_width = if let Length::Fixed(width) = self.element.widget().size_hint().width {
            width
        } else {
            self.element.widget().size().width
        };
        let content_height = if let Length::Fixed(height) = self.element.widget().size_hint().height
        {
            height
        } else {
            self.element.widget().size().height
        };
        let mut whole_buffer = Buffer::new(content_width, content_height);
        self.element.widget().render(
            Area::new(0, 0, content_width, content_height),
            &mut whole_buffer,
            theme,
        );
        let cut_buffer = whole_buffer.cut(Area::new(
            self.state.offset.x,
            self.state.offset.y,
            area.width
                .saturating_sub(if self.should_vertical_scroll(area.height) {
                    1
                } else {
                    0
                }),
            area.height
                .saturating_sub(if self.should_horizontal_scroll(area.width) {
                    1
                } else {
                    0
                }),
        ));
        buffer.render(area.x, area.y, &cut_buffer);

        // Render the scrollbars.
        if self.should_vertical_scroll(area.height) && cut_buffer.width() < area.width {
            buffer.fill(
                self.vertical_draggable_bounds(area, content_height),
                Pixel::char_with_style(' ', Style::new().background(Color::Primary)),
            );
        }
        if self.should_horizontal_scroll(area.width) && cut_buffer.height() < area.height {
            buffer.fill(
                self.horizontal_draggable_bounds(area, content_width),
                Pixel::char_with_style(' ', Style::new().background(Color::Primary)),
            );
        }
    }

    fn size(&self) -> Size {
        self.element.widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.size_hint
    }
}

impl<'a, Message: Clone + 'a> From<Scrollable<'a, Message>> for Element<'a, Message> {
    fn from(scrollable: Scrollable<'a, Message>) -> Self {
        Self::new(scrollable)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_draggable_to_offset() {
        let bar_bounds = Area::new(0, 0, 1, 9);
        let draggable_bounds = Area::new(0, 3, 1, 3);
        let element_height = 20;

        let offset = Scrollable::<()>::vertical_draggable_to_offset(bar_bounds, draggable_bounds, element_height);
        assert_eq!(offset, 6);
    }
}