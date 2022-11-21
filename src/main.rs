mod framework;
mod game;
mod tests;

use framework::{Game, Pos};
use game::{GameImpl};

fn main() {
    let game = GameImpl::new().unwrap();
    game.print();
    let game = game.move_(Pos('B',2),Pos('B',3)).unwrap();
    game.print();
}