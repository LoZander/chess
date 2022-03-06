use crate::framework::*;
use crate::framework::{Piece::*, Player::*};

pub struct GameImpl {
    pieces_grid: Vec<Vec<Piece>>,
}

pub fn build_GameImpl() -> GameImpl {
    GameImpl {
        pieces_grid: vec![vec![Rook(White), Knight(White),  Bishop(White),  King(White), Queen(White),  Bishop(White), Knight(White), Rook(White)],
                          vec![Pawn(White), Pawn(White),    Pawn(White),    Pawn(White), Pawn(White),   Pawn(White),   Pawn(White),   Pawn(White)],
                          vec![None,        None,           None,           None,        None,          None,          None,          None],
                          vec![None,        None,           None,           None,        None,          None,          None,          None],
                          vec![None,        None,           None,           None,        None,          None,          None,          None],
                          vec![None,        None,           None,           None,        None,          None,          None,          None],
                          vec![Pawn(Black), Pawn(Black),    Pawn(Black),    Pawn(Black), Pawn(Black),   Pawn(Black),   Pawn(Black),   Pawn(Black)],
                          vec![Rook(Black), Knight(Black),  Bishop(Black),  King(Black), Queen(Black),  Bishop(Black), Knight(Black), Rook(Black)]]
    }
}

impl Game for GameImpl {
    fn move_(&mut self, from: &Position, to: &Position) -> bool { 
        let to_x_pos = number_from_char(to.x_pos);
        let to_y_pos = to.y_pos;

        let piece = extract_piece(self, from);
        self.pieces_grid[to_y_pos - 1][to_x_pos - 1] = piece;
        return true;
    }
    fn get_piece(&self, position: &Position) -> &Piece {
        let x_pos = number_from_char(position.x_pos);
        let y_pos = position.y_pos;
        let piece = &self.pieces_grid[y_pos - 1][x_pos - 1];
        return piece;
    }
}

fn extract_piece(game: &mut GameImpl, position: &Position) -> Piece {
    let x_pos = number_from_char(position.x_pos);
    let y_pos = position.y_pos;
    let replace = game.pieces_grid[y_pos - 1].remove(x_pos - 1);
    game.pieces_grid[y_pos - 1].insert(x_pos - 1,None);
    return replace;
}



fn number_from_char(c: char) -> usize {
    match c {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'D' => 4,
        'E' => 5,
        'F' => 6,
        'G' => 7,
        'H' => 8,
        _ => panic!()
    }
}