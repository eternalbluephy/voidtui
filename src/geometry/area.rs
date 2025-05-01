use super::{size::Size, spacing::Spacing};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Area {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Area {
    pub const fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Area {
            x,
            y,
            width,
            height,
        }
    }

    pub const fn zeros() -> Self {
        Area {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }

    /// Creates a new area with a size, starting at the (0, 0) corner.
    pub const fn from_size(size: Size) -> Self {
        Area {
            x: 0,
            y: 0,
            width: size.width,
            height: size.height,
        }
    }

    pub const fn from_wh(width: u16, height: u16) -> Self {
        Area {
            x: 0,
            y: 0,
            width,
            height,
        }
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

    pub fn shrink(&self, padding: impl Into<Spacing>) -> Self {
        let padding: Spacing = padding.into();
        Area::new(
            self.x + padding.left,
            self.y + padding.top,
            self.width.saturating_sub(padding.left + padding.right),
            self.height.saturating_sub(padding.top + padding.bottom),
        )
    }

    pub fn expand(&self, padding: impl Into<Spacing>) -> Self {
        let padding: Spacing = padding.into();
        Area::new(
            self.x.saturating_sub(padding.left),
            self.y.saturating_sub(padding.top),
            self.width.saturating_add(padding.left + padding.right),
            self.height.saturating_add(padding.top + padding.bottom),
        )
    }

    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    pub fn up(&self, offset: u16) -> Self {
        Area::new(self.x, self.y.saturating_sub(offset), self.width, self.height)
    }

    pub fn right(&self, offset: u16) -> Self {
        Area::new(self.x.saturating_add(offset), self.y, self.width, self.height)
    }

    pub fn down(&self, offset: u16) -> Self {
        Area::new(self.x, self.y.saturating_add(offset), self.width, self.height)
    }

    pub fn left(&self, offset: u16) -> Self {
        Area::new(self.x.saturating_add(offset), self.y, self.width, self.height)
    }

    pub fn x(&self, x: u16) -> Self {
        Area::new(x, self.y, self.width, self.height)
    }

    pub fn y(&self, y: u16) -> Self {
        Area::new(self.x, y, self.width, self.height)
    }

    pub fn min(&self, size: Size) -> Self {
        Area::new(
            self.x,
            self.y,
            size.width.min(self.width),
            size.height.min(self.height),
        )
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
