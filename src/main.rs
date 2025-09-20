mod ball;
mod paddle;
mod score;
mod timer;

use ball::{Ball, BounceDirection, CollisionEvent};
use macroquad::prelude::*;
use paddle::Paddle;
use score::Score;
use timer::Timer;

pub struct PongGame {
    ball: Ball,
    paddle_1: Paddle,
    paddle_2: Paddle,
    score: Score,
    timer: Timer,
}

impl PongGame {
    pub fn new() -> Self {
        PongGame {
            ball: Ball::new(),
            paddle_1: Paddle::new(60.0, 20.0),
            paddle_2: Paddle::new(screen_width() - 60.0, 20.0),
            score: Score::new(),
            timer: Timer::new(),
        }
    }

    pub fn handle_movement(&mut self) {
        self.ball.update_position();
        self.ball.handle_vertical_collisions();
        if self.ball.overlaps_with_paddle(&self.paddle_1) {
            self.ball.handle_active_collision(BounceDirection::Right);
        }
        if self.ball.overlaps_with_paddle(&self.paddle_2) {
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
            self.paddle_1.move_up()
        }
        if is_key_down(KeyCode::S) {
            self.paddle_1.move_down()
        }

        if is_key_down(KeyCode::Up) {
            self.paddle_2.move_up()
        }
        if is_key_down(KeyCode::Down) {
            self.paddle_2.move_down()
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
