extern crate ggez;

use ggez::{event, Context, ContextBuilder, GameResult};
use snake::constants::{SCREEN_SIZE};
use snake::game::Game;

fn main() -> GameResult {
    let (ctx, events_loop) = ContextBuilder::new("snake", "BoyanVD")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("Failed to build context !");

    let game = Game::new();
    
    event::run(ctx, events_loop, game)
}
