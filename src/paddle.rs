use macroquad::prelude::*;

pub const PADDLE_SPEED: f32 = 600.0;

pub struct Paddle {
    pub rect: Rect,
}

impl Paddle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            rect: Rect::new(x, y, 20.0, 100.0),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }

    pub fn update(&mut self, dt: f32) {
        if is_key_down(KeyCode::Down) && self.rect.y + self.rect.h < screen_height() {
            self.rect.y += 1.0 * dt * PADDLE_SPEED;
        }
        if is_key_down(KeyCode::Up) && self.rect.y > 0.0 {
            self.rect.y -= 1.0 * dt * PADDLE_SPEED;
        }
    }
}
