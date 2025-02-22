#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spacing {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

impl Spacing {
    pub const fn new(top: u16, right: u16, bottom: u16, left: u16) -> Self {
        Spacing {
            top,
            right,
            bottom,
            left,
        }
    }

    pub const fn axes(y: u16, x: u16) -> Self {
        Spacing {
            top: y,
            right: x,
            bottom: y,
            left: x,
        }
    }

    pub const fn vertical(y: u16) -> Self {
        Spacing {
            top: y,
            right: 0,
            bottom: y,
            left: 0,
        }
    }

    pub const fn horizontal(x: u16) -> Self {
        Spacing {
            top: 0,
            right: x,
            bottom: 0,
            left: x,
        }
    }

    pub const fn all(value: u16) -> Self {
        Spacing {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    pub const fn zeros() -> Self {
        Spacing {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
        }
    }
}

impl From<u16> for Spacing {
    fn from(value: u16) -> Self {
        Spacing::all(value)
    }
}

impl From<(u16, u16)> for Spacing {
    fn from((y, x): (u16, u16)) -> Self {
        Spacing::axes(y, x)
    }
}

impl From<(u16, u16, u16, u16)> for Spacing {
    fn from((top, right, bottom, left): (u16, u16, u16, u16)) -> Self {
        Spacing::new(top, right, bottom, left)
    }
}
