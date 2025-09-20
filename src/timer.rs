use macroquad::color::WHITE;
use macroquad::prelude::{draw_text, get_time, measure_text, screen_height, screen_width};

pub struct Timer {
    start_time: f32,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start_time: get_time() as f32,
        }
    }

    fn round_duration(&self) -> f32 {
        (get_time() as f32) - self.start_time
    }

    pub fn reset(&mut self) {
        self.start_time = get_time() as f32
    }

    pub fn draw(&self) {
        let total_seconds = self.round_duration() as u32;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        let time_text = format!("{:02}:{:02}", minutes, seconds);
        let text_dimensions = measure_text(&time_text, None, 30, 1.0);
        let x_position = (screen_width() - text_dimensions.width) / 2.0;
        draw_text(&time_text, x_position, screen_height() - 50.0, 30.0, WHITE);
    }
}
