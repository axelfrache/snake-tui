use super::{Board, Position, Snake};
use rand::Rng;

pub struct Food {
    position: Position,
}

impl Food {
    pub fn new(board: &Board, snake: &Snake) -> Self {
        let mut rng = rand::thread_rng();
        let mut position;
        loop {
            position = Position {
                x: rng.gen_range(1..board.width - 1),
                y: rng.gen_range(1..board.height - 1),
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
