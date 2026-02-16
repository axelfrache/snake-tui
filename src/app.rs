use crate::game::{Board, Food, Snake};
use crate::highscore::HighScore;

pub struct App {
    pub snake: Snake,
    pub food: Food,
    pub board: Board,
    pub score: u32,
    pub high_score: HighScore,
    pub game_over: bool,
    pub paused: bool,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> App {
        let board = Board::new(20, 20);
        let snake = Snake::new(10, 10);
        let food = Food::new(&board, &snake);
        let high_score = HighScore::load();

        App {
            snake,
            food,
            board,
            score: 0,
            high_score,
            game_over: false,
            paused: false,
            should_quit: false,
        }
    }

    pub fn tick(&mut self) {
        if self.game_over || self.paused {
            return;
        }

        if !self.snake.move_forward(&self.board) {
            self.game_over = true;
            if self.score > self.high_score.best_score {
                self.high_score.best_score = self.score;
                self.high_score.save();
            }
            return;
        }

        if self.snake.head() == self.food.position() {
            self.snake.grow();
            self.score += 1;
            self.food = Food::new(&self.board, &self.snake);
        }
    }

    pub fn toggle_pause(&mut self) {
        if !self.game_over {
            self.paused = !self.paused;
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new(10, 10);
        self.board = Board::new(20, 20);
        self.food = Food::new(&self.board, &self.snake);
        self.score = 0;
        self.game_over = false;
        self.paused = false;
        self.should_quit = false;
    }
}
