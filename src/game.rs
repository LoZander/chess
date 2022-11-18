use std::collections::HashMap;

use crate::framework::*;
use crate::framework::{Piece::*, Player::*};


pub struct GameImpl {
    pieces_grid: HashMap<Position, Piece>,
}

fn piece_entry (x_pos: char, y_pos: usize, piece: Piece) -> (Position, Piece) { (Position {x_pos, y_pos}, piece) }

pub fn build_game_impl() -> GameImpl {
    GameImpl {
        pieces_grid: HashMap::from([
            piece_entry ('A', 1, Rook(White)),
            piece_entry ('B', 1, Knight(White)),
            piece_entry ('C', 1, Bishop(White)),
            piece_entry ('D', 1, King(White)),
            piece_entry ('E', 1, Queen(White)),
            piece_entry ('F', 1, Bishop(White)),
            piece_entry ('G', 1, Knight(White)),
            piece_entry ('H', 1, Rook(White)),
            
            piece_entry ('A', 2, Pawn(White)),
            piece_entry ('B', 2, Pawn(White)),
            piece_entry ('C', 2, Pawn(White)),
            piece_entry ('D', 2, Pawn(White)),
            piece_entry ('E', 2, Pawn(White)),
            piece_entry ('F', 2, Pawn(White)),
            piece_entry ('G', 2, Pawn(White)),
            piece_entry ('H', 2, Pawn(White)),

            piece_entry ('A', 8, Rook(Black)),
            piece_entry ('B', 8, Knight(Black)),
            piece_entry ('C', 8, Bishop(Black)),
            piece_entry ('D', 8, King(Black)),
            piece_entry ('E', 8, Queen(Black)),
            piece_entry ('F', 8, Bishop(Black)),
            piece_entry ('G', 8, Knight(Black)),
            piece_entry ('H', 8, Rook(Black)),
            
            piece_entry ('A', 7, Pawn(Black)),
            piece_entry ('B', 7, Pawn(Black)),
            piece_entry ('C', 7, Pawn(Black)),
            piece_entry ('D', 7, Pawn(Black)),
            piece_entry ('E', 7, Pawn(Black)),
            piece_entry ('F', 7, Pawn(Black)),
            piece_entry ('G', 7, Pawn(Black)),
            piece_entry ('H', 7, Pawn(Black)),
            ])
        }
    }
    
impl Game for GameImpl {
    type Game=GameImpl;
    fn move_(mut self, from: Position, to: Position) -> Result<Self::Game,String> {
        let piece = self.pieces_grid.remove(&from);
        match piece {
            None => Err(format!("No piece at {}", from)),
            Some(p) => {
                self.pieces_grid.insert(to, p); 
                Ok(self)
            }
        }
    }

    fn get_piece(&self, position: Position) -> Option<&Piece> {
        self.pieces_grid.get(&position)
    }

}