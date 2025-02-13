use unicode_width::UnicodeWidthChar;

use crate::style::{color::Color, style::Style};

#[derive(Debug, Clone)]
pub struct Pixel {
  character: Option<char>,
  width: u8,
  style: Style
}

impl Pixel {
  /// Create an empty pixel.
  pub const fn new() -> Self {
    Pixel {
      character: None,
      width: 1,
      style: Style::new()
    }
  }

  /// Create a pixel with a character.
  pub fn from_char(character: char) -> Self {
    let width = character.width().unwrap() as u8;
    Pixel {
      character: Some(character),
      width,
      style: Style::new()
    }
  }

  /// Create a pixel with a character and set the width.
  /// Ensure the width is correct, otherwise the rendering will be incorrect.
  pub fn from_charw(character: char, width: u8) -> Self {
    Pixel {
      character: Some(character),
      width,
      style: Style::new()
    }
  }

  pub fn set_character(&mut self, character: char) -> &Self {
    self.character = Some(character);
    self.width = character.width().unwrap() as u8;
    self
  }

  pub fn width(&self) -> u8 {
    self.width
  }

  pub fn set_style(&mut self, style: Style) -> &mut Self {
    self.style = style;
    self
  }

  pub fn set_background(&mut self, background: Color) -> &mut Self {
    self.style.background = Some(background);
    self
  }

  /// Clear the character and style.
  pub fn clear(&mut self) -> &mut Self {
    self.character = None;
    self.width = 1;
    self.style = Style::new();
    self
  }

  /// Clear the character only.
  pub fn clear_char(&mut self) -> &mut Self {
    self.character = None;
    self.width = 1;
    self
  }

  /// Set the pixel as skipped, the pixel will not be rendered.
  /// This is usually used after a two-width character.
  pub fn set_skip(&mut self) -> &mut Self {
    self.character = Some(char::from_u32(0).unwrap());
    self.width = 0;
    self
  }

  pub fn is_skip(&self) -> bool {
    self.character.is_some() && self.character.unwrap() as u32 == 0
  }

  /// Render another pixel on this pixel.
  pub fn render(&mut self, pixel: &Pixel) -> &mut Self {
    if pixel.style.background.is_some() {
      self.character = pixel.character;
      self.width = pixel.width;
      self.style = pixel.style.clone();
    } else if pixel.character.is_some() {
      self.character = pixel.character;
      self.width = pixel.width;
      self.style.foreground = pixel.style.foreground;
      self.style.attributes = pixel.style.attributes;
    }
    self
  }

  pub fn character(&self) -> Option<char> {
    self.character
  }

  pub fn style(&self) -> Style {
    self.style.clone()
  }

  pub fn style_ref(&self) -> &Style {
    &self.style
  }

  pub fn style_mut(&mut self) -> &mut Style {
    &mut self.style
  }

}