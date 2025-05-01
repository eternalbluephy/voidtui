#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
    
    pub fn up(&self, offset: u16) -> Self {
        Self { x: self.x, y: self.y.saturating_sub(offset) }
    }

    pub fn right(&self, offset: u16) -> Self {
        Self { x: self.x.saturating_add(offset), y: self.y }
    }

    pub fn down(&self, offset: u16) -> Self {
        Self { x: self.x, y: self.y.saturating_add(offset) }
    }

    pub fn left(&self, offset: u16) -> Self {
        Self { x: self.x.saturating_sub(offset), y: self.y }
    }
}

impl From<(u16, u16)> for Point {
    fn from(value: (u16, u16)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}