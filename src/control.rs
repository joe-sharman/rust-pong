use macroquad::prelude::*;

pub enum Action {
    Up,
    Down,
    Stop,
}

pub trait BaseControl {
    fn resolve_movement(&self, ball_in_half: bool, ball_y: f32, paddle_y: f32) -> Action;
}

struct KeyboardControl {
    up_key: KeyCode,
    down_key: KeyCode,
}

impl BaseControl for KeyboardControl {
    fn resolve_movement(&self, _: bool, _: f32, _: f32) -> Action {
        if is_key_down(self.up_key) {
            Action::Up
        } else if is_key_down(self.down_key) {
            Action::Down
        } else {
            Action::Stop
        }
    }
}

struct AutomaticControl {}

impl BaseControl for AutomaticControl {
    fn resolve_movement(&self, ball_in_half: bool, ball_y: f32, paddle_y: f32) -> Action {
        // If ball isn't in half don't react, makes it easier for players
        if !ball_in_half {
            return Action::Stop;
        }
        // Move towards the ball, stop if close to avoid jittering
        if paddle_y > ball_y + 10.0 {
            Action::Up
        } else if paddle_y < ball_y - 10.0 {
            Action::Down
        } else {
            Action::Stop
        }
    }
}

pub fn new_control(keys: Option<(KeyCode, KeyCode)>) -> Box<dyn BaseControl> {
    match keys {
        Some((up_key, down_key)) => Box::new(KeyboardControl { up_key, down_key }),
        None => Box::new(AutomaticControl {}),
    }
}
