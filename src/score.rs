use macroquad::prelude::*;

const WINNING_SCORE: i32 = 5;

pub struct Score {
    pub player1_score: i32,
    pub player2_score: i32,
}

impl Score {
    pub fn new() -> Self {
        Self {
            player1_score: 0,
            player2_score: 0,
        }
    }

    pub fn reset(&mut self) {
        self.player1_score = 0;
        self.player2_score = 0;
    }

    pub fn increase_player1_score(&mut self) {
        self.player1_score += 1;
    }

    pub fn increase_player2_score(&mut self) {
        self.player2_score += 1;
    }

    pub fn check_winner(&self) -> Option<i32> {
        if self.player1_score >= WINNING_SCORE {
            Some(1)
        } else if self.player2_score >= WINNING_SCORE {
            Some(2)
        } else {
            None
        }
    }

    pub fn draw(&self) {
        draw_text_ex(
            &self.player1_score.to_string(),
            screen_width() / 2. - 200.0,
            50.0,
            TextParams {
                font_size: 60,
                color: WHITE,
                ..Default::default()
            },
        );
        draw_text_ex(
            &self.player2_score.to_string(),
            screen_width() / 2. + 200.0,
            50.0,
            TextParams {
                font_size: 60,
                color: WHITE,
                ..Default::default()
            },
        );
    }
}
