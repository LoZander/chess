mod framework;
mod game;
mod tests;

use game::build_game_impl;

fn main() {
    let _game = build_game_impl();
}