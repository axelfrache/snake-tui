use super::{Board, Direction, Position};
use std::collections::VecDeque;

pub struct Snake {
    body: VecDeque<Position>,
    direction: Direction,
    direction_queue: VecDeque<Direction>,
    growing: bool,
}

impl Snake {
    pub fn new(x: u16, y: u16) -> Self {
        let mut body = VecDeque::new();
        body.push_back(Position { x, y });
        body.push_back(Position { x: x - 1, y });
        body.push_back(Position { x: x - 2, y });

        Self {
            body,
            direction: Direction::Right,
            direction_queue: VecDeque::new(),
            growing: false,
        }
    }

    pub fn head(&self) -> Position {
        *self.body.front().unwrap()
    }

    pub fn body(&self) -> &VecDeque<Position> {
        &self.body
    }

    pub fn set_direction(&mut self, direction: Direction) {
        let last_direction = self.direction_queue.back().unwrap_or(&self.direction);

        if direction != last_direction.opposite() && direction != *last_direction {
            if self.direction_queue.len() < 2 {
                self.direction_queue.push_back(direction);
            }
        }
    }

    pub fn move_forward(&mut self, board: &Board) -> bool {
        if let Some(next_dir) = self.direction_queue.pop_front() {
            self.direction = next_dir;
        }

        let head = self.head();
        let new_head = match self.direction {
            Direction::Up => {
                if head.y == 0 {
                    return false;
                }
                Position {
                    x: head.x,
                    y: head.y - 1,
                }
            }
            Direction::Down => Position {
                x: head.x,
                y: head.y + 1,
            },
            Direction::Left => {
                if head.x == 0 {
                    return false;
                }
                Position {
                    x: head.x - 1,
                    y: head.y,
                }
            }
            Direction::Right => Position {
                x: head.x + 1,
                y: head.y,
            },
        };

        if !board.is_inside(new_head) || self.body.contains(&new_head) {
            return false;
        }

        self.body.push_front(new_head);
        if !self.growing {
            self.body.pop_back();
        } else {
            self.growing = false;
        }

        true
    }

    pub fn grow(&mut self) {
        self.growing = true;
    }
}
