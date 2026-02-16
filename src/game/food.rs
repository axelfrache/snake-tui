use super::{Board, Position, Snake};
use rand::random_range;

pub struct Food {
    position: Position,
}

impl Food {
    pub fn new(board: &Board, snake: &Snake) -> Self {
        let mut position;
        loop {
            position = Position {
                x: random_range(1..board.width - 1),
                y: random_range(1..board.height - 1),
            };

            if !snake.body().contains(&position) {
                break;
            }
        }
        Self { position }
    }

    pub fn position(&self) -> Position {
        self.position
    }
}
