use super::{attributes::Attributes, color::Color, color::ColorSystem, theme::Theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub attributes: Attributes,
}

impl Style {
    /// Create an empty style.
    pub const fn new() -> Self {
        Style {
            foreground: None,
            background: None,
            attributes: Attributes::new(),
        }
    }

    /// Create a style only with a foreground color.
    pub fn from_foreground<T: Into<Color>>(foreground: T) -> Self {
        Style {
            foreground: Some(foreground.into()),
            background: None,
            attributes: Attributes::new(),
        }
    }

    /// Create a style only with a background color.
    pub fn from_background<T: Into<Color>>(background: T) -> Self {
        Style {
            foreground: None,
            background: Some(background.into()),
            attributes: Attributes::new(),
        }
    }

    /// Clear the style.
    pub fn clear(mut self) -> Self {
        self.foreground = None;
        self.background = None;
        self.attributes.clear();
        self
    }

    /// Set the foreground color.
    pub fn foreground<T: Into<Color>>(mut self, color: T) -> Self {
        self.foreground = Some(color.into());
        self
    }

    /// Set the background color.
    pub fn background<T: Into<Color>>(mut self, color: T) -> Self {
        self.background = Some(color.into());
        self
    }

    /// Set the attributes set.
    pub fn attributes(mut self, attributes: Attributes) -> Self {
        self.attributes = attributes;
        self
    }

    pub fn bold(mut self) -> Self {
        self.attributes.bold();
        self
    }

    pub fn dim(mut self) -> Self {
        self.attributes.dim();
        self
    }

    pub fn italic(mut self) -> Self {
        self.attributes.italic();
        self
    }

    pub fn underline(mut self) -> Self {
        self.attributes.underline();
        self
    }

    pub fn blink(mut self) -> Self {
        self.attributes.blink();
        self
    }

    pub fn reverse(mut self) -> Self {
        self.attributes.reverse();
        self
    }

    pub fn conceal(mut self) -> Self {
        self.attributes.conceal();
        self
    }

    pub fn strike(mut self) -> Self {
        self.attributes.strike();
        self
    }

    pub fn frame(mut self) -> Self {
        self.attributes.frame();
        self
    }

    pub fn encircle(mut self) -> Self {
        self.attributes.encircle();
        self
    }

    pub fn overline(mut self) -> Self {
        self.attributes.overline();
        self
    }

    pub fn ansi_codes(&self, system: ColorSystem, theme: &Theme) -> String {
        let mut codes = Vec::new();
        if system != ColorSystem::Disabled {
            if let Some(foreground) = self.foreground {
                codes.push(foreground.on_theme(theme).ansi_codes(system, true));
            }
            if let Some(background) = self.background {
                codes.push(background.on_theme(theme).ansi_codes(system, false));
            }
        }
        if !self.attributes.is_empty() {
            codes.push(self.attributes.ansi_codes());
        }
        codes.join(";")
    }
}

#[cfg(test)]
mod tests {
    use crate::style::{color::Color, color::ColorSystem, theme::Theme};

    use super::Style;

    #[test]
    fn ansi_codes() {
        let style = Style::new()
            .foreground(Color::Text)
            .background(Color::Background)
            .strike();
        println!(
            "\x1b[{}mHello, world!\x1b[0m",
            style.ansi_codes(ColorSystem::TrueColor, &Theme::TOKYO_NIGHT)
        );
    }
}
