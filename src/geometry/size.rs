#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size<T = u16> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub const fn new(width: T, height: T) -> Self {
        Size { width, height }
    }
}

impl<T> From<(T, T)> for Size<T> {
    fn from(value: (T, T)) -> Self {
        Size {
            width: value.0,
            height: value.1,
        }
    }
}

impl<T> Into<(T, T)> for Size<T> {
    fn into(self) -> (T, T) {
        (self.width, self.height)
    }
}
