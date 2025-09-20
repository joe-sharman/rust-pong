use macroquad::color::BLUE;
use macroquad::prelude::{draw_line, screen_height};

pub struct Paddle {
    x: f32,
    y: f32,
}

impl Paddle {
    const LENGTH: f32 = 100.0;
    const THICKNESS: f32 = 15.0;
    const SPEED: f32 = 8.0;

    pub fn new(x: f32, y: f32) -> Self {
        Paddle { x, y }
    }

    pub fn move_up(&mut self) {
        self.y = (self.y - Self::SPEED).max(0.0);
    }

    pub fn move_down(&mut self) {
        self.y = (self.y + Self::SPEED).min(screen_height() - Self::LENGTH);
    }

    pub fn draw(&self) {
        draw_line(
            self.x,
            self.y,
            self.x,
            self.y + Self::LENGTH,
            Self::THICKNESS,
            BLUE,
        )
    }

    pub fn left_side(&self) -> f32 {
        self.x - Paddle::THICKNESS / 2.0
    }
    pub fn right_side(&self) -> f32 {
        self.x + Paddle::THICKNESS / 2.0
    }
    pub fn top_side(&self) -> f32 {
        self.y
    }
    pub fn bottom_side(&self) -> f32 {
        self.y + Paddle::LENGTH
    }
}
