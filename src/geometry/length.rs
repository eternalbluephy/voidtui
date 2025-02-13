pub enum Length {
  Fixed(u16),
  Fill
}

impl From<u16> for Length {
  fn from(value: u16) -> Self {
    Length::Fixed(value)
  }
}