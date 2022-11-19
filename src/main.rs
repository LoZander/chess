mod framework;
mod game;
mod tests;

use framework::{Game, Pos};
use game::{build_game_impl, print_board};

fn main() {
    let game = build_game_impl();
    print_board(&game);
    let game = game.move_(Pos('B',2),Pos('B',3)).unwrap();
    print_board(&game);
}