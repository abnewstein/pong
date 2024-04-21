use macroquad::prelude::*;

mod ball;
mod paddle;
use ball::Ball;
use paddle::Paddle;

#[macroquad::main("pong")]
async fn main() {
    let mut player1 = Paddle::new(10.0, screen_height() / 2. - 50.);
    let mut player2 = Paddle::new(screen_width() - 30.0, screen_height() / 2. - 50.);
    let mut ball = Ball::new(screen_width() / 2., screen_height() / 2.);
    loop {
        let dt = get_frame_time();
        clear_background(BLACK);

        player1.update(dt);
        player2.update(dt);
        ball.update(dt);
        ball.collision_check(&player1, &player2);

        player1.draw();
        player2.draw();
        ball.draw();

        next_frame().await
    }
}
