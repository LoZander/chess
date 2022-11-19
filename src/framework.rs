pub trait Game {
    type Game;
    fn move_(self, from: Pos, to: Pos) -> Result<Self::Game,String>;
    fn get_piece(&self, position: Pos) -> Option<&Piece>;
}

pub trait Gui {
    fn run(&self);
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Piece {
    Pawn(Player),
    Rook(Player),
    Knight(Player),
    Bishop(Player),
    King(Player),
    Queen(Player)
}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum Player {
    White,
    Black,
}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Clone, Copy)]
pub struct Pos(pub char, pub i8);

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}