use macroquad::color::BLUE;
use macroquad::prelude::{draw_line, screen_height};

pub trait BasePaddle {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn length(&self) -> f32;
    fn thickness(&self) -> f32;
    fn draw(&self) {
        draw_line(
            self.x(),
            self.y(),
            self.x(),
            self.y() + self.length(),
            self.thickness(),
            BLUE,
        )
    }

    fn left_side(&self) -> f32 {
        self.x() - self.thickness() / 2.0
    }
    fn right_side(&self) -> f32 {
        self.x() + self.thickness() / 2.0
    }
    fn top_side(&self) -> f32 {
        self.y()
    }
    fn bottom_side(&self) -> f32 {
        self.y() + self.length()
    }

    fn move_up(&mut self);

    fn move_down(&mut self);

    fn stop(&mut self);

    fn centre_y(&self) -> f32 {
        self.y() + (self.length() / 2.0)
    }
}

#[derive(Clone, Debug)]
struct Paddle {
    x: f32,
    y: f32,
}

impl Paddle {
    const LENGTH: f32 = 100.0;
    const THICKNESS: f32 = 15.0;
    const SPEED: f32 = 5.0;
    pub fn new(x: f32, y: f32) -> Self {
        Paddle { x, y }
    }
}

impl BasePaddle for Paddle {
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
    }
    fn length(&self) -> f32 {
        Self::LENGTH
    }
    fn thickness(&self) -> f32 {
        Self::THICKNESS
    }

    fn move_up(&mut self) {
        self.y = (self.y - Self::SPEED).max(0.0);
    }

    fn move_down(&mut self) {
        self.y = (self.y + Self::SPEED).min(screen_height() - Self::LENGTH);
    }

    fn stop(&mut self) {}
}

#[derive(Clone, Debug)]
struct PaddleWithMomentum {
    x: f32,
    y: f32,
    velocity: f32,
}

impl PaddleWithMomentum {
    const LENGTH: f32 = 100.0;
    const THICKNESS: f32 = 15.0;

    fn update_position(&mut self) {
        self.y = (self.y + self.velocity)
            .max(0.0)
            .min(screen_height() - Self::LENGTH);
    }
    pub fn new(x: f32, y: f32) -> Self {
        let velocity = 0.0;
        PaddleWithMomentum { x, y, velocity }
    }
}

impl BasePaddle for PaddleWithMomentum {
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
    }
    fn length(&self) -> f32 {
        Self::LENGTH
    }
    fn thickness(&self) -> f32 {
        Self::THICKNESS
    }

    fn move_up(&mut self) {
        self.velocity = (self.velocity - 0.5).max(-10.0);
        self.update_position();
    }

    fn move_down(&mut self) {
        self.velocity = (self.velocity + 0.5).min(10.0);
        self.update_position();
    }

    fn stop(&mut self) {
        if self.velocity.abs() <= 0.5 {
            self.velocity = 0.0;
        } else {
            self.velocity -= self.velocity.signum() * 0.5;
        }
        self.update_position();
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PaddleVariant {
    Basic,
    Momentum,
}

pub fn new_paddle(x: f32, y: f32, variant: PaddleVariant) -> Box<dyn BasePaddle> {
    match variant {
        PaddleVariant::Basic => Box::new(Paddle::new(x, y)),
        PaddleVariant::Momentum => Box::new(PaddleWithMomentum::new(x, y)),
    }
}
