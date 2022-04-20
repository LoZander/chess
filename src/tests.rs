#![cfg(test)]
use crate::game::{build_GameImpl};
use crate::framework::*;
use crate::framework::{Game, Position, Piece::*, Player::{White}};

#[test]
fn should_be_white_rook_at_1A() {
    let game = build_GameImpl();
    let pos = Position {
        x_pos: 'A', 
        y_pos: 1,
    };

    let piece = game.get_piece(&pos);
    assert_eq!(piece, &Piece::Rook(White));
}

#[test]
fn should_be_white_knight_at_1B() {
    let game = build_GameImpl();
    let pos = Position {
        x_pos: 'B',
        y_pos: 1,
    };

    let piece = game.get_piece(&pos);
    assert_eq!(piece, &Piece::Knight(White))
}

#[test]
fn getting_piece_shouldnt_remove_it() {
    let game = build_GameImpl();
    let pos = Position {
        x_pos: 'B',
        y_pos: 1,
    };
    let piece1 = game.get_piece(&pos);
    let piece2 = game.get_piece(&pos);
    assert_eq!(piece1,piece2);
}

#[test]
fn white_pawn_should_move() {
    let mut game = build_GameImpl();
    let from = Position {x_pos: 'A', y_pos: 2};
    let to = Position {x_pos: 'A', y_pos: 3};
    game.move_(&from, &to);

    let piece = game.get_piece(&to);
    let none = game.get_piece(&from);
    assert_eq!(piece, &Piece::Pawn(White));
    assert_eq!(none, &None);
}