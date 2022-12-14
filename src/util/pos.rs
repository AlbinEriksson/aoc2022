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
