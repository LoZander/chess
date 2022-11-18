#![cfg(test)]
use crate::game::{build_game_impl};
use crate::framework::{Game, Position, Piece::*, Player::{White}};

#[test]
fn should_be_white_rook_at_1_a() {
    let game = build_game_impl();
    let pos = Position {
        x_pos: 'A', 
        y_pos: 1,
    };

    let piece = game.get_piece(pos);
    assert_eq!(piece, Some(&Rook(White)));
}

#[test]
fn should_be_white_knight_at_1_b() {
    let game = build_game_impl();
    let pos = Position {
        x_pos: 'B',
        y_pos: 1,
    };

    let piece = game.get_piece(pos);
    assert_eq!(piece, Some(&Knight(White)))
}

#[test]
fn getting_piece_shouldnt_remove_it() {
    let game = build_game_impl();
    let pos = Position {
        x_pos: 'B',
        y_pos: 1,
    };
    let piece1 = game.get_piece(pos);
    let piece2 = game.get_piece(pos);
    assert_eq!(piece1,piece2);


}

#[test]
fn white_pawn_should_move() {
    let game = build_game_impl();
    let from = Position {x_pos: 'A', y_pos: 2};
    let to = Position {x_pos: 'A', y_pos: 3};
    let game = game.move_(from, to).unwrap();

    let piece = game.get_piece(to);
    let none = game.get_piece(from);
    assert_eq!(piece, Some(&Pawn(White)));
    assert_eq!(none, None);
}