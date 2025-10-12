mod ball;
mod paddle;
mod score;
mod timer;

use ball::{Ball, BounceDirection, CollisionEvent};
use macroquad::prelude::*;
use paddle::{BasePaddle, PaddleVariant, new_paddle};
use score::Score;
use timer::Timer;

pub struct PongGame {
    ball: Ball,
    paddle_1: Box<dyn BasePaddle>,
    paddle_2: Box<dyn BasePaddle>,
    score: Score,
    timer: Timer,
}

impl PongGame {
    pub fn new() -> Self {
        PongGame {
            ball: Ball::new(),
            paddle_1: new_paddle(60.0, 20.0, PaddleVariant::Momentum),
            paddle_2: new_paddle(screen_width() - 60.0, 20.0, PaddleVariant::Momentum),
            score: Score::new(),
            timer: Timer::new(),
        }
    }

    pub fn handle_movement(&mut self) {
        self.ball.update_position();
        self.ball.handle_vertical_collisions();
        if self.ball.overlaps_with_paddle(&*self.paddle_1) {
            self.ball.handle_active_collision(BounceDirection::Right);
        }
        if self.ball.overlaps_with_paddle(&*self.paddle_2) {
            self.ball.handle_active_collision(BounceDirection::Left);
        }
    }

    pub fn check_if_scored(&mut self) {
        if let Some(collision) = self.ball.check_side_wall_collisions() {
            match collision {
                CollisionEvent::LeftWallCollision => self.score.increase_player2(),
                CollisionEvent::RightWallCollision => self.score.increase_player1(),
            }
            self.ball.reset();
            self.timer.reset();
        }
    }

    pub fn handle_input(&mut self) {
        if is_key_down(KeyCode::W) {
            self.paddle_1.move_up();
        } else if is_key_down(KeyCode::S) {
            self.paddle_1.move_down();
        } else {
            self.paddle_1.stop();
        }

        if is_key_down(KeyCode::Up) {
            self.paddle_2.move_up();
        } else if is_key_down(KeyCode::Down) {
            self.paddle_2.move_down();
        } else {
            self.paddle_2.stop();
        }
    }

    pub fn draw(&self) {
        self.paddle_1.draw();
        self.paddle_2.draw();
        self.ball.draw();
        self.score.draw();
        self.timer.draw();
    }
}

#[macroquad::main("Pong Game")]
async fn main() {
    let mut game = PongGame::new();

    loop {
        game.draw();
        game.handle_input();
        game.handle_movement();
        game.check_if_scored();
        next_frame().await;
    }
}
