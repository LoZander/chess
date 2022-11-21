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
                    Some(p) => {map.insert(Pos(i as i8 + 1,j as i8 + 1), p); ()}
                }
            }
        }
        Ok(GameImpl{pieces_grid: map})
    }    
    pub fn print(self: &GameImpl) {
        for i in (1..9).rev() {
            for j in 1..9 {
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
            match from - to {
                Pos(1,2) |
                Pos(-1,2) |
                Pos(1,-2) |
                Pos(-1,-2) |
                Pos(2,1) |
                Pos(-2,1) |
                Pos(2,-1) |
                Pos(-2,-1) => Ok(p),
                _ => Err(format!("Illegal move: Knight cannot move in such a manner"))
            }
        Bishop(_) => {
            let dif = from - to;
            if dif.0 == dif.1 || dif.0 == -dif.1 {
                Ok(p)
            } else {
                Err(format!("Illegal move: Bishop cannot move in such a manner"))
            }
        }

        Rook(_) => match from - to {
            Pos(_,0) |
            Pos(0,_) => Ok(p),
            _ => Err(format!("Illegal move: Rook cannot move in such a manner"))
        }

        Queen(_) => match from - to {
            Pos(_,0) |
            Pos(0,_) => Ok(p),
            Pos(n,m) if n == m || n == -m => Ok(p),
            _ => Err(format!("Illegal move: Queen cannot move in such a manner"))
        }
        _ => panic!()
    }
}