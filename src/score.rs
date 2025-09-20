use macroquad::color::WHITE;
use macroquad::prelude::{draw_text, measure_text, screen_width};

pub struct Score {
    player1: u32,
    player2: u32,
}

impl Score {
    pub fn new() -> Self {
        Score {
            player1: 0,
            player2: 0,
        }
    }

    pub fn increase_player1(&mut self) {
        self.player1 += 1;
    }

    pub fn increase_player2(&mut self) {
        self.player2 += 1;
    }

    pub fn draw(&self) {
        let score_text = format!("{}  -  {}", self.player1, self.player2);
        let text_dimensions = measure_text(&score_text, None, 30, 1.0);
        let x_position = (screen_width() - text_dimensions.width) / 2.0;
        draw_text(&score_text, x_position, 50.0, 30.0, WHITE);
    }
}
