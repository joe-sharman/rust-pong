use crate::BasePaddle;
use macroquad::color::GREEN;
use macroquad::math::clamp;
use macroquad::prelude::{draw_circle, screen_height, screen_width};
use macroquad::rand::gen_range;

pub enum CollisionEvent {
    LeftWallCollision,
    RightWallCollision,
}

pub enum BounceDirection {
    Left,
    Right,
}
pub struct Ball {
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,
}

impl Ball {
    const RADIUS: f32 = 30.0;

    pub fn new() -> Self {
        let (vx, vy) = Self::get_rand_velocity();
        Ball {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            velocity_x: vx,
            velocity_y: vy,
        }
    }

    fn get_rand_velocity() -> (f32, f32) {
        let angle = gen_range(0.0, std::f32::consts::TAU); // Random angle 0-360°
        let speed = 2.0; // Total speed magnitude, adjust to change starting speed

        let velocity_x = speed * angle.cos();
        let velocity_y = speed * angle.sin();

        (velocity_x, velocity_y)
    }

    fn randomise_velocity(&mut self) {
        let (vx, vy) = Self::get_rand_velocity();
        self.velocity_x = vx;
        self.velocity_y = vy;
    }

    pub fn reset(&mut self) {
        self.x = screen_width() / 2.0;
        self.y = screen_height() / 2.0;
        self.randomise_velocity()
    }

    pub fn overlaps_left_wall(&self) -> bool {
        self.x <= Self::RADIUS
    }

    pub fn overlaps_right_wall(&self) -> bool {
        self.x >= screen_width() - Self::RADIUS
    }

    pub fn update_position(&mut self) {
        self.y = (self.y - self.velocity_y)
            .max(0.0 + Self::RADIUS)
            .min(screen_height() - Self::RADIUS);
        self.x = (self.x - self.velocity_x)
            .max(0.0 + Self::RADIUS)
            .min(screen_width() - Self::RADIUS);
    }

    pub fn handle_vertical_collisions(&mut self) {
        // Inverts either y if ball is touching top or bottom wall.
        if self.y == Self::RADIUS {
            self.velocity_y *= -1.0;
        }
        if self.y == screen_height() - Self::RADIUS {
            self.velocity_y *= -1.0;
        }
    }

    pub fn overlaps_with_paddle(&self, paddle: &dyn BasePaddle) -> bool {
        // Checks if the ball overlaps with a paddle.
        // Finds closet point on paddle to ball then calculates distance
        // using pythagoras and compares to ball radius.
        // Ball shouldn't depend on Paddle here. This method should be moved to a separate
        // module that manages physics/collisions between different objects.
        let paddle_closest_x = clamp(self.x, paddle.left_side(), paddle.right_side());
        let paddle_closest_y = clamp(self.y, paddle.top_side(), paddle.bottom_side());

        let distance =
            ((self.x - paddle_closest_x).powf(2.0) + (self.y - paddle_closest_y).powf(2.0)).sqrt();

        distance <= Self::RADIUS
    }

    pub fn handle_active_collision(&mut self, direction: BounceDirection) {
        // Reacts to a collision with active bounce direction.
        // Randomly adjusts y velocity for unpredictability.
        // Increases x speec slightly so game speed increases.
        match direction {
            BounceDirection::Left => self.velocity_x = self.velocity_x.abs(),
            BounceDirection::Right => self.velocity_x = -self.velocity_x.abs(),
        }
        self.velocity_x *= 1.0 + 0.08; // Slightly increase speed on bounce
        // Randomly adjust y velocity slightly to make it more interesting
        self.velocity_y += gen_range(-0.5, 0.5);
    }

    pub fn check_side_wall_collisions(&self) -> Option<CollisionEvent> {
        // Returns a collision event if ball is touching either side wall.
        if self.overlaps_left_wall() {
            Some(CollisionEvent::LeftWallCollision)
        } else if self.overlaps_right_wall() {
            Some(CollisionEvent::RightWallCollision)
        } else {
            None
        }
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, Self::RADIUS, GREEN);
    }
}
