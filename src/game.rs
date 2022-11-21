use std::collections::HashMap;

use crate::framework::*;
use crate::framework::{Piece::*, Player::*};

#[derive(Debug)]
pub struct GameImpl {
    pieces_grid: HashMap<Pos, Piece>
}

impl GameImpl {
    pub fn new() -> Result<Self,String> {
        Self::build_game(&[
            "RNBKQBNR",
            "PPPPPPPP",
            "        ",
            "        ",
            "        ",
            "        ",
            "pppppppp",
            "rnbkqbnr",
            ])
    }
    pub fn build_game(pieces: &[&'static str] ) -> Result<GameImpl,String> {
        let mut map: HashMap<Pos,Piece> = HashMap::new();
        for (j,s) in pieces.iter().rev().enumerate() {
            for (i,c) in s.char_indices() {
                let p = piece_from_char(c);
                match p {
                    None => (),
                    Some(p) => {map.insert(Pos::from(i as i8 + 1,j as i8 + 1)?, p); ()}
                }
            }
        }
        Ok(GameImpl{pieces_grid: map})
    }    
    pub fn print(self: &GameImpl) {
        for i in (1..9).rev() {
            for j in 'A'..'I' {
                let pos = Pos(j, i);
                match self.get_piece(pos) {
                    None => print!(" "),
                    Some(p) => print!("{p}")
                };
                print!("  ")
            }
            print!("\n")
        }
    }
}

fn piece_from_char(c: char) -> Option<Piece> {
    match c {
        'p' => Some(Pawn(White)),
        'P' => Some(Pawn(Black)),
        'r' => Some(Rook(White)),
        'R' => Some(Rook(Black)),
        'b' => Some(Bishop(White)),
        'B' => Some(Bishop(Black)),
        'n' => Some(Knight(White)),
        'N' => Some(Knight(Black)),
        'k' => Some(King(White)),
        'K' => Some(King(Black)),
        'q' => Some(Queen(White)),
        'Q' => Some(Queen(Black)),
        _ => None
    }
}
    
impl Game for GameImpl {
    type Game = GameImpl;
    fn move_(self, from: Pos, to: Pos) -> Result<Self,String> {
        let (game, piece) = remove(from, self)?;
        let piece = is_move_valid(from,to,piece)?;
        let game = add(to, piece, game);
        Ok(game)
    }

    fn get_piece(&self, position: Pos) -> Option<&Piece> {
        self.pieces_grid.get(&position)
    }
}

fn add(pos: Pos, piece: Piece, mut game: GameImpl) -> GameImpl {
    game.pieces_grid.insert(pos, piece);
    game
}

fn remove(from: Pos, mut game: GameImpl) -> Result<(GameImpl,Piece),String> {
    let piece = game.pieces_grid.remove(&from).ok_or(format!("No piece at {}", from))?;
    Ok((game, piece))
}

fn is_move_valid (from: Pos, to: Pos, p: Piece) -> Result<Piece,String> {
    let int_from = int_pos(from)?;
    let int_to = int_pos(to)?;
    match p {
        Pawn(White) => 
            if from.0 == to.0 {
                Ok(p)
            } else {
                Err(format!("Illegal move: Pawn cannot move in such a manner"))
            }.and_then(|p|
                if from.1 < to.1 {
                    Ok(p)
                } else {
                    Err(format!("Illegal move: Pawn cannot move in such a manner"))
                }),
        Knight(_) =>
            match (int_to.0 - int_from.0, int_to.1 - int_from.1) {
                (1,2) |
                (-1,2) |
                (1,-2) |
                (-1,-2) |
                (2,1) |
                (-2,1) |
                (2,-1) |
                (-2,-1) => Ok(p),
                _ => Err(format!("Illegal move: Knight cannot move in such a manner"))
            }
        Bishop(_) => {
            let dif_0 = int_to.0 - int_from.0;
            let dif_1 = int_to.1 - int_from.1;
            if dif_0 == dif_1 || dif_0 == -dif_1 {
                Ok(p)
            } else {
                Err(format!("Illegal move: Bishop cannot move in such a manner"))
            }
        }

        Rook(_) => match (int_to.0 - int_from.0, int_to.1 - int_from.1) {
            (_,0) |
            (0,_) => Ok(p),
            _ => Err(format!("Illegal move: Rook cannot move in such a manner"))
        }

        Queen(_) => {
            let dif_0 = int_to.0 - int_from.0;
            let dif_1 = int_to.1 - int_from.1;
            if dif_0 == dif_1 || dif_0 == -dif_1 {
                Ok(p)
            } else {
                Err(format!("Illegal move: Bishop cannot move in such a manner"))
            }
        }
        _ => panic!()
    }
}

fn int_pos(Pos(x,y): Pos) -> Result<(i8, i8), String> {
    match x {
        'A' => Ok(1),
        'B' => Ok(2),
        'C' => Ok(3),
        'D' => Ok(4),
        'E' => Ok(5),
        'F' => Ok(6),
        'G' => Ok(7),
        'H' => Ok(8),
        c => Err(format!("Cannot translate '{}' into a number as it's not a valid chessboard letter", c))
    }.map(|x| (x,y))
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