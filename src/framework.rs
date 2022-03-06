pub trait Game {
    fn move_(&mut self, from: &Position, to: &Position) -> bool;
    fn get_piece(&self, position: &Position) -> &Piece;
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Piece {
    Pawn(Player),
    Rook(Player),
    Knight(Player),
    Bishop(Player),
    King(Player),
    Queen(Player),
    None,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Player {
    White,
    Black,
}


pub struct Position {
    pub x_pos: char,
    pub y_pos: usize,
}