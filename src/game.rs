use macroquad::prelude::*;

use crate::ball::Ball;
use crate::paddle::Paddle;
use crate::score::Score;

enum GameState {
    MainMenu,
    Running,
    GameOver,
}

pub struct Game {
    player1: Paddle,
    player2: Paddle,
    ball: Ball,
    state: GameState,
    score: Score,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player1: Paddle::new(20.0, screen_height() / 2. - 50.),
            player2: Paddle::new(screen_width() - 40.0, screen_height() / 2. - 50.),
            ball: Ball::new(screen_width() / 2., screen_height() / 2.),
            state: GameState::MainMenu,
            score: Score::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let state = &mut self.state;
        let player1 = &mut self.player1;
        let player2 = &mut self.player2;
        let ball = &mut self.ball;
        let score = &mut self.score;

        match state {
            GameState::MainMenu => {
                draw_text(
                    "Press Space to start",
                    screen_width() / 2. - 100.0,
                    screen_height() / 2. - 50.0,
                    30.0,
                    WHITE,
                );
                if is_key_down(KeyCode::Space) {
                    *state = GameState::Running;
                }
            }
            GameState::Running => {
                player1.update(dt);
                player2.update(dt);
                ball.update(dt);
                ball.collision_check(player1, player2);

                if ball.circle.x - ball.circle.r < 0.0 {
                    score.increase_player2_score();
                    *ball = Ball::new(screen_width() / 2., screen_height() / 2.);
                } else if ball.circle.x + ball.circle.r > screen_width() {
                    score.increase_player1_score();
                    *ball = Ball::new(screen_width() / 2., screen_height() / 2.);
                }

                if score.check_winner().is_some() {
                    *state = GameState::GameOver;
                }

                player1.draw();
                player2.draw();
                ball.draw();
                score.draw();
            }
            GameState::GameOver => {
                if let Some(winner) = score.check_winner() {
                    draw_text(
                        format!("Player {winner} wins").as_str(),
                        screen_width() / 2. - 100.0,
                        screen_height() / 2. - 100.0,
                        30.0,
                        WHITE,
                    );
                }
                draw_text(
                    "Game Over",
                    screen_width() / 2. - 100.0,
                    screen_height() / 2. - 50.0,
                    30.0,
                    WHITE,
                );
                draw_text(
                    "Press Space to restart",
                    screen_width() / 2. - 100.0,
                    screen_height() / 2.,
                    30.0,
                    WHITE,
                );
                if is_key_down(KeyCode::Space) {
                    *player1 = Paddle::new(20.0, screen_height() / 2. - 50.);
                    *player2 = Paddle::new(screen_width() - 40.0, screen_height() / 2. - 50.);
                    *ball = Ball::new(screen_width() / 2., screen_height() / 2.);
                    score.reset();
                    *state = GameState::Running;
                }
            }
        }
    }
}
