#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Attributes {
  value: u16
}

impl Attributes {
  /// Create an empty set of attributes.
  pub const fn new() -> Self {
    Attributes { value: 0 }
  }

  pub fn clear(&mut self) -> Self {
    self.value = 0;
    *self
  }

  pub fn bold(&mut self) -> Self {
    self.value |= 1;
    *self
  }

  pub fn dim(&mut self) -> Self {
    self.value |= 2;
    *self
  }

  pub fn italic(&mut self) -> Self {
    self.value |= 4;
    *self
  }

  pub fn underline(&mut self) -> Self {
    self.value |= 8;
    *self
  }

  pub fn blink(&mut self) -> Self {
    self.value |= 16;
    *self
  }

  pub fn reverse(&mut self) -> Self {
    self.value |= 32;
    *self
  }

  pub fn conceal(&mut self) -> Self {
    self.value |= 64;
    *self
  }

  pub fn strike(&mut self) -> Self {
    self.value |= 128;
    *self
  }

  pub fn frame(&mut self) -> Self {
    self.value |= 256;
    *self
  }

  pub fn encircle(&mut self) -> Self {
    self.value |= 512;
    *self
  }

  pub fn overline(&mut self) -> Self {
    self.value |= 1024;
    *self
  }

  pub fn is_empty(&self) -> bool {
    self.value == 0
  }

  pub fn ansi_codes(&self) -> String {
    let mut codes = Vec::new();
    if self.value & 1 != 0 {
      codes.push("1");
    }
    if self.value & 2 != 0 {
      codes.push("2");
    }
    if self.value & 4 != 0 {
      codes.push("3");
    }
    if self.value & 8 != 0 {
      codes.push("4");
    }
    if self.value & 16 != 0 {
      codes.push("5");
    }
    if self.value & 32 != 0 {
      codes.push("7");
    }
    if self.value & 64 != 0 {
      codes.push("8");
    }
    if self.value & 128 != 0 {
      codes.push("9");
    }
    if self.value & 256 != 0 {
      codes.push("51");
    }
    if self.value & 512 != 0 {
      codes.push("52");
    }
    if self.value & 1024 != 0 {
      codes.push("53");
    }
    codes.join(";")
  }
}


#[cfg(test)]
mod tests {
    use super::Attributes;

  #[test]
  fn ansi_codes() {
    let attributes = Attributes::new().italic().strike().underline();
    println!("\x1b[{}mHello, world!\x1b[0m", attributes.ansi_codes());
  }
}