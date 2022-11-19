#![cfg(test)]
use crate::framework::Pos;
use crate::game::{build_game_impl};
use crate::framework::{Game, Piece::*, Player::{White}};

#[test]
fn should_be_white_rook_at_1_a() {
    let game = build_game_impl();
    let pos = Pos('A',1);

    let piece = game.get_piece(pos);
    assert_eq!(piece, Some(&Rook(White)));
}

#[test]
fn should_be_white_knight_at_1_b() {
    let game = build_game_impl();
    let pos = Pos('B',1);

    let piece = game.get_piece(pos);
    assert_eq!(piece, Some(&Knight(White)))
}

#[test]
fn getting_piece_shouldnt_remove_it() {
    let game = build_game_impl();
    let pos = Pos('B',1);
    let piece1 = game.get_piece(pos);
    let piece2 = game.get_piece(pos);
    assert_eq!(piece1,piece2);


}

#[test]
fn white_pawn_should_move() {
    let game = build_game_impl();
    let from = Pos('A',2);
    let to = Pos('A',3);
    let game = game.move_(from, to).unwrap();

    let piece = game.get_piece(to);
    let none = game.get_piece(from);
    assert_eq!(piece, Some(&Pawn(White)));
    assert_eq!(none, None);
}

#[test]
fn white_pawn_cant_move_sideways() {
    let game = build_game_impl();
    let game = game.move_(Pos('B',2),Pos('B',3)).unwrap();
    match game.move_(Pos('B',3), Pos('C',3)) {
        Ok (_)  => assert!(false),
        Err (e) => assert_eq!("Illegal move: Pawn cannot move in such a manner", e) 
    }
}

#[test]
fn white_pawn_cant_move_backwards() {
    let game = build_game_impl();
    let game = game.move_(Pos('B',2),Pos('B',3)).unwrap();
    match game.move_(Pos('B',3),Pos('B',2)) {
        Ok (_) => assert!(false),
        Err (e) => assert_eq!("Illegal move: Pawn cannot move in such a manner", e)
    }
}