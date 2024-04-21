use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

use crate::paddle::Paddle;

const BALL_SPEED: f32 = 200.0;

pub struct Ball {
    circle: Circle,
    velocity: Vec2,
}

enum CollisionSide {
    Top,
    Bottom,
    Sides,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        let mut rng = thread_rng();
        let x_dir = rng.gen_range(-1.0..-0.5);
        let y_dir = rng.gen_range(0.5..1.);
        Self {
            circle: Circle::new(x, y, 10.0),
            velocity: Vec2::new(x_dir, y_dir).normalize(),
        }
    }

    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, WHITE);
    }

    pub fn update(&mut self, dt: f32) {
        self.circle.x += self.velocity.x * dt * BALL_SPEED;
        self.circle.y += self.velocity.y * dt * BALL_SPEED;

        if self.circle.y + self.circle.r > screen_height() || self.circle.y - self.circle.r < 0.0 {
            self.velocity.y *= -1.0;
        }
    }

    fn collides_with(&self, paddle: &Paddle) -> Option<CollisionSide> {
        let paddle_top = paddle.rect.y;
        let paddle_bottom = paddle.rect.y + paddle.rect.h;
        let paddle_left = paddle.rect.x;
        let paddle_right = paddle.rect.x + paddle.rect.w;

        let ball_top = self.circle.y - self.circle.r;
        let ball_bottom = self.circle.y + self.circle.r;
        let ball_left = self.circle.x - self.circle.r;
        let ball_right = self.circle.x + self.circle.r;

        if ball_left > paddle_right || ball_bottom < paddle_top || ball_top > paddle_bottom {
            return None;
        }
        if ball_left < paddle_right && ball_right > paddle_left {
            if ball_top > paddle_top && ball_bottom < paddle_bottom {
                return Some(CollisionSide::Sides);
            } else if ball_top < paddle_top && ball_bottom > paddle_top {
                return Some(CollisionSide::Top);
            } else if ball_bottom > paddle_bottom && ball_top < paddle_bottom {
                return Some(CollisionSide::Bottom);
            }
        }
        None
    }

    pub fn collision_check(&mut self, paddle1: &Paddle, paddle2: &Paddle) {
        self.handle_paddle_collision(paddle1);
        self.handle_paddle_collision(paddle2);
    }

    fn handle_paddle_collision(&mut self, paddle: &Paddle) {
        if let Some(side) = self.collides_with(paddle) {
            match side {
                CollisionSide::Sides => {
                    self.velocity.x *= -1.0;
                }
                CollisionSide::Top | CollisionSide::Bottom => {
                    self.velocity.y *= -1.0;
                }
            }
        }
    }
}
