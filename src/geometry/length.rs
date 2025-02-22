pub enum Length {
    Fixed(u16),
    Part(u16),
    Shrink,
    Fill
}