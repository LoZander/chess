use std::collections::HashMap;

use crate::framework::*;
use crate::framework::{Piece::*, Player::*};

#[derive(Debug)]
pub struct GameImpl {
    pieces_grid: HashMap<Pos, Piece>
}

pub fn build_game_impl() -> GameImpl {
    GameImpl {
        pieces_grid: HashMap::from([
            (Pos('A', 1), Rook(White)),
            (Pos('B', 1), Knight(White)),
            (Pos('C', 1), Bishop(White)),
            (Pos('D', 1), Queen(White)),
            (Pos('E', 1), King(White)),
            (Pos('F', 1), Bishop(White)),
            (Pos('G', 1), Knight(White)),
            (Pos('H', 1), Rook(White)),

            (Pos('A', 2), Pawn(White)),
            (Pos('B', 2), Pawn(White)),
            (Pos('C', 2), Pawn(White)),
            (Pos('D', 2), Pawn(White)),
            (Pos('E', 2), Pawn(White)),
            (Pos('F', 2), Pawn(White)),
            (Pos('G', 2), Pawn(White)),
            (Pos('H', 2), Pawn(White)),
            (Pos('A', 8), Rook(Black)),
            (Pos('B', 8), Knight(Black)),
            (Pos('C', 8), Bishop(Black)),
            (Pos('D', 8), Queen(Black)),
            (Pos('E', 8), King(Black)),
            (Pos('F', 8), Bishop(Black)),
            (Pos('G', 8), Knight(Black)),
            (Pos('H', 8), Rook(Black)),

            (Pos('A', 7), Pawn(Black)),
            (Pos('B', 7), Pawn(Black)),
            (Pos('C', 7), Pawn(Black)),
            (Pos('D', 7), Pawn(Black)),
            (Pos('E', 7), Pawn(Black)),
            (Pos('F', 7), Pawn(Black)),
            (Pos('G', 7), Pawn(Black)),
            (Pos('H', 7), Pawn(Black)),
            ])
        }
    }
    
impl Game for GameImpl {
    type Game = GameImpl;
    fn move_(mut self, from: Pos, to: Pos) -> Result<Self,String> {
        let piece = self.pieces_grid.remove(&from);
        piece.ok_or(format!("No piece at {}", from))
             .and_then(|x| 
                if from.0 == to.0 {
                    Ok(x)
                } else {
                    Err(format!("Illegal move: Pawn cannot move in such a manner"))
                })
             .and_then(|p|
                if from.1 <= to.1 {
                    Ok(p)
                } else {
                    Err(format!("Illegal move: Pawn cannot move in such a manner"))
                })
             .map(|x| {self.pieces_grid.insert(to, x); self})
    }

    fn get_piece(&self, position: Pos) -> Option<&Piece> {
        self.pieces_grid.get(&position)
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let symbol = match self {
            King(Black) => '\u{2654}',
            Queen(Black) => '\u{2655}',
            Rook(Black) => '\u{2656}',
            Bishop(Black) => '\u{2657}',
            Knight(Black) => '\u{2658}',
            Pawn(Black) => '\u{2659}',
            King(White) => '\u{265A}',
            Queen(White) => '\u{265B}',
            Rook(White) => '\u{265C}',
            Bishop(White) => '\u{265D}',
            Knight(White) => '\u{265E}',
            Pawn(White) => '\u{265F}',
        };

        write!(f, "{symbol}")
    }
}

pub fn print_board(game: &GameImpl) {
    for i in (1..9).rev() {
        for j in 'A'..'I' {
            let pos = Pos(j, i);
            match game.get_piece(pos) {
                None => print!(" "),
                Some(p) => print!("{p}")
            };
            print!("  ")
        }
        print!("\n")
    }
}