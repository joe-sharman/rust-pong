use crate::control::{Action, new_control, BaseControl};
use crate::paddle::{PaddleVariant, new_paddle, BasePaddle};
use macroquad::prelude::*;

pub enum Side {
    Left,
    Right,
}

pub struct Player {
    pub paddle: Box<dyn BasePaddle>,
    control: Box<dyn BaseControl>,
    side: Side,
}

impl Player {
    pub fn new(
        side: Side,
        paddle_variant: PaddleVariant,
        keys: Option<(KeyCode, KeyCode)>,
    ) -> Self {
        let x = match side {
            Side::Left => 60.0,
            Side::Right => screen_width() - 60.0,
        };
        let paddle = new_paddle(x, 20.0, paddle_variant);
        let control = new_control(keys);
        Player {
            paddle,
            control,
            side,
        }
    }

    fn is_ball_in_half(&self, ball_x: f32) -> bool {
        match self.side {
            Side::Left => ball_x < (screen_width() / 2.0),
            Side::Right => ball_x > (screen_width() / 2.0),
        }
    }

    pub fn update_position(&mut self, ball_x: f32, ball_y: f32) {
        let action = self.control.resolve_movement(
            self.is_ball_in_half(ball_x),
            ball_y,
            self.paddle.centre_y(),
        );
        match action {
            Action::Up => self.paddle.move_up(),
            Action::Down => self.paddle.move_down(),
            Action::Stop => self.paddle.stop(),
        }
    }
}
