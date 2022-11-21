#![cfg(test)]
use crate::framework::Pos;
use crate::framework::{Game, Piece::*, Player::{White}};
use crate::game::GameImpl;

#[test]
fn should_be_white_rook_at_1_a() {
    let game = GameImpl::new().unwrap();
    let pos = Pos('A',1);

    let piece = game.get_piece(pos);
    assert_eq!(piece, Some(&Rook(White)));
}

#[test]
fn should_be_white_knight_at_1_b() {
    let game = GameImpl::new().unwrap();
    let pos = Pos('B',1);

    let piece = game.get_piece(pos);
    assert_eq!(piece, Some(&Knight(White)))
}

#[test]
fn getting_piece_shouldnt_remove_it() {
    let game = GameImpl::new().unwrap();
    let pos = Pos('B',1);
    let piece1 = game.get_piece(pos);
    let piece2 = game.get_piece(pos);
    assert_eq!(piece1,piece2);


}

#[test]
fn white_pawn_should_move() {
    let game = GameImpl::new().unwrap();
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
    let game = GameImpl::new().unwrap();
    let game = game.move_(Pos('B',2),Pos('B',3)).unwrap();
    match game.move_(Pos('B',3), Pos('C',3)) {
        Ok (_)  => assert!(false),
        Err (e) => assert_eq!("Illegal move: Pawn cannot move in such a manner", e) 
    }
}

#[test]
fn white_pawn_cant_move_backwards() {
    let game = GameImpl::new().unwrap();
    let game = game.move_(Pos('B',2),Pos('B',3)).unwrap();
    match game.move_(Pos('B',3),Pos('B',2)) {
        Ok (_) => assert!(false),
        Err (e) => assert_eq!("Illegal move: Pawn cannot move in such a manner", e)
    }
}

#[test]
fn white_knight_can_move_in_l_shape() {
    let game = GameImpl::new().unwrap();
    let game = game.move_(Pos('G',1), Pos('F',3)).unwrap();
    
    match game.get_piece(Pos('F',3)) {
        None => assert!(false),
        Some(p) => assert_eq!(&Knight(White), p)
    }
}

#[test]
fn white_knight_cant_move_forward() {
    let game = GameImpl::new().unwrap();
    match game.move_(Pos('G',1), Pos('G',2)) {
        Err(e) => assert_eq!("Illegal move: Knight cannot move in such a manner",e),
        Ok(_) => assert!(false)
    }
}

#[test]
fn white_bishop_can_move_diagonally() {
    let game = GameImpl::build_game(&[
        "--------", // 8
        "--------", // 7
        "--------", // 6
        "---b----", // 5
        "--------", // 4
        "--------", // 3
        "--------", // 2
        "--------", // 1
    //   ABCDEFGH
    ]).unwrap();
    let game = game.move_(Pos('D',5), Pos('B',7)).unwrap();

    let p = game.get_piece(Pos('B',7)).expect("Expected: white bishop\nGot: none");
    assert_eq!(&Bishop(White), p)
}

#[test]
fn white_bishop_cant_move_horizontally() {
    let game = GameImpl::build_game(&[
        "--------", // 8
        "--------", // 7
        "--------", // 6
        "---b----", // 5
        "--------", // 4
        "--------", // 3
        "--------", // 2
        "--------", // 1
    //   ABCDEFGH
    ]).unwrap();
    match game.move_(Pos('D',5), Pos('B',5)) {
        Err(e) => assert_eq!("Illegal move: Bishop cannot move in such a manner",e),
        Ok(_) => panic!()
    }
}

#[test]
fn white_rook_can_move_vertically() {
    let game = GameImpl::build_game(&[
        "--------", // 8
        "--------", // 7
        "--------", // 6
        "---r----", // 5
        "--------", // 4
        "--------", // 3
        "--------", // 2
        "--------", // 1
    //   ABCDEFGH
    ]).unwrap();
    let game = game.move_(Pos('D',5), Pos('D',8)).unwrap();

    let p = game.get_piece(Pos('D',8)).expect("Expected: white rook\nGot: none");
    assert_eq!(&Rook(White), p)
}

#[test]
fn white_queen_can_move_diagonally() {
    let game = GameImpl::build_game(&[
        "--------", // 8
        "--------", // 7
        "--------", // 6
        "---q----", // 5
        "--------", // 4
        "--------", // 3
        "--------", // 2
        "--------", // 1
    //   ABCDEFGH
    ]).unwrap();
    let game = game.move_(Pos('D',5), Pos('B',7)).unwrap();

    let p = game.get_piece(Pos('B',7)).expect("Expected: white queen\nGot: none");
    assert_eq!(&Queen(White), p)
}

#[test]
fn white_queen_can_move_vertically() {
    let game = GameImpl::build_game(&[
        "--------", // 8
        "--------", // 7
        "--------", // 6
        "---q----", // 5
        "--------", // 4
        "--------", // 3
        "--------", // 2
        "--------", // 1
    //   ABCDEFGH
    ]).unwrap();
    let game = game.move_(Pos('D',5), Pos('D',2)).unwrap();

    let p = game.get_piece(Pos('D',2)).expect("Expected: white queen\nGot: none");
    assert_eq!(&Queen(White), p)
}

#[test]
fn white_queen_cant_move_arbitrarily() {
    let game = GameImpl::build_game(&[
        "--------", // 8
        "--------", // 7
        "--------", // 6
        "---q----", // 5
        "--------", // 4
        "--------", // 3
        "--------", // 2
        "--------", // 1
    //   ABCDEFGH
    ]).unwrap();
    match game.move_(Pos('D',5), Pos('E',8)) {
        Err(e) => assert_eq!("Illegal move: Queen cannot move in such a manner",e),
        Ok(_) => panic!()
    }
}