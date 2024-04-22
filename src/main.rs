use macroquad::prelude::*;

mod ball;
mod game;
mod paddle;
mod score;

use game::Game;

#[macroquad::main("pong")]
async fn main() {
    let mut game = Game::new();
    loop {
        let dt = get_frame_time();
        clear_background(BLACK);
        game.update(dt);

        next_frame().await
    }
}
