use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Copy, Default, Hash)]
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

#[derive(PartialEq, Eq, Clone, Copy, Default, Hash)]
pub struct Pos3d<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Pos3d<T> {
    pub fn new(x: T, y: T, z: T) -> Pos3d<T> {
        Pos3d { x, y, z }
    }
}

impl<T: Display> Display for Pos3d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
