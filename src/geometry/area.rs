#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Area {
  pub x: u16,
  pub y: u16,
  pub width: u16,
  pub height: u16
}

impl Area {
  pub const fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
    Area { x, y, width, height }
  }

  pub fn is_empty(&self) -> bool {
    self.width == 0 || self.height == 0
  }

  pub fn intersect(&self, other: Self) -> Self {
    let x1 = self.x.max(other.x);
    let y1 = self.y.max(other.y);
    let x2 = u16::min(self.x + self.width, other.x + other.width);
    let y2 = u16::min(self.y + self.height, other.y + other.height);

    if x2 <= x1 || y2 <= y1 {
      Area::new(0, 0, 0, 0)
    } else {
      Area::new(x1, y1, x2 - x1, y2 - y1)
    }
  }

  pub fn corners(&self) -> (u16, u16, u16, u16) {
    (self.x, self.y, self.x + self.width, self.y + self.height)
  }
}


#[cfg(test)]
mod tests {
    use super::Area;

  #[test]
  fn intersect() {
    let area1 = Area::new(0, 0, 10, 10);
    let area2 = Area::new(5, 5, 10, 10);
    let area3 = Area::new(20, 20, 10, 10);
    assert_eq!(area1.intersect(area2), Area::new(5, 5, 5, 5));
    assert_eq!(area1.intersect(area3), Area::new(0, 0, 0, 0));
  }
}