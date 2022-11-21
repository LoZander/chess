mod framework;
mod game;
mod tests;

use framework::{Game, Pos};
use game::{GameImpl};

fn main() {
    let game = GameImpl::new().unwrap();
    game.print();
    let game = game.move_(Pos::from('B',2).unwrap(),Pos::from('B',3).unwrap()).unwrap();
    game.print();
}