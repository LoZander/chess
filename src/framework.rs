pub trait Game {
    type Game;
    fn move_(self, from: Position, to: Position) -> Result<Self::Game,String>;
    fn get_piece(&self, position: Position) -> Option<&Piece>;
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


#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Clone, Copy)]
pub struct Position {
    pub x_pos: char,
    pub y_pos: usize,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.x_pos, self.y_pos)
    }
}