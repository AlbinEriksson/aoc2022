use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub struct Pos2d<T> {
    pub x: T,
    pub y: T
}

impl<T> Pos2d<T> {
    pub fn new(x: T, y: T) -> Pos2d<T> {
        Pos2d { x, y }
    }
}

impl<T: Display> Display for Pos2d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
