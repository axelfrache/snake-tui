#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Board {
    pub width: u16,
    pub height: u16,
}

impl Board {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn is_inside(&self, pos: Position) -> bool {
        pos.x < self.width && pos.y < self.height
    }
}
