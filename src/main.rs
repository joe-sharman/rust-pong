mod ball;
mod control;
mod paddle;
mod player;
mod score;
mod timer;
use crate::player::Side;
use ball::{Ball, BounceDirection, CollisionEvent};
use macroquad::prelude::*;
use paddle::{PaddleVariant};
use player::Player;
use score::Score;
use timer::Timer;

pub struct PongGame {
    ball: Ball,
    player_1: Player,
    player_2: Player,
    score: Score,
    timer: Timer,
}

impl PongGame {
    pub fn new() -> Self {
        PongGame {
            ball: Ball::new(),
            player_1: Player::new(
                Side::Left,
                PaddleVariant::Momentum,
                Some((KeyCode::W, KeyCode::S)),
            ),
            player_2: Player::new(Side::Right, PaddleVariant::Basic, None),
            score: Score::new(),
            timer: Timer::new(),
        }
    }

    pub fn handle_movement(&mut self) {
        self.ball.update_position();
        self.ball.handle_vertical_collisions();
        if self.ball.overlaps_with_paddle(&*self.player_1.paddle) {
            self.ball.handle_active_collision(BounceDirection::Right);
        }
        if self.ball.overlaps_with_paddle(&*self.player_2.paddle) {
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
        self.player_1.update_position(self.ball.x, self.ball.y);
        self.player_2.update_position(self.ball.x, self.ball.y);
    }

    pub fn draw(&self) {
        self.player_1.paddle.draw();
        self.player_2.paddle.draw();
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
