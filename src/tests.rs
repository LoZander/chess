#![cfg(test)]
use crate::framework::Pos;
use crate::framework::{Game, Piece::*, Player::{White}};
use crate::game::GameImpl;

#[test]
fn should_be_white_rook_at_1_a() {
    let game = GameImpl::new().unwrap();
    let pos = Pos::from('A',1).unwrap();

    let piece = game.get_piece(pos);
    assert_eq!(piece, Some(&Rook(White)));
}

#[test]
fn should_be_white_knight_at_1_b() {
    let game = GameImpl::new().unwrap();
    let pos = Pos::from('B',1).unwrap();

    let piece = game.get_piece(pos);
    assert_eq!(piece, Some(&Knight(White)))
}

#[test]
fn getting_piece_shouldnt_remove_it() {
    let game = GameImpl::new().unwrap();
    let pos = Pos::from('B',1).unwrap();
    let piece1 = game.get_piece(pos);
    let piece2 = game.get_piece(pos);
    assert_eq!(piece1,piece2);


}

#[test]
fn white_pawn_should_move() {
    let game = GameImpl::new().unwrap();
    let from = Pos::from('A',2).unwrap();
    let to = Pos::from('A',3).unwrap();
    let game = game.move_(from, to).unwrap();

    let piece = game.get_piece(to);
    let none = game.get_piece(from);
    assert_eq!(piece, Some(&Pawn(White)));
    assert_eq!(none, None);
}

#[test]
fn white_pawn_cant_move_sideways() {
    let game = GameImpl::new().unwrap();
    let game = game.move_(Pos::from('B',2).unwrap(),Pos::from('B',3).unwrap()).unwrap();
    match game.move_(Pos::from('B',3).unwrap(), Pos::from('C',3).unwrap()) {
        Ok (_)  => assert!(false),
        Err (e) => assert_eq!("Illegal move: Pawn cannot move in such a manner", e) 
    }
}

#[test]
fn white_pawn_cant_move_backwards() {
    let game = GameImpl::new().unwrap();
    let game = game.move_(Pos::from('B',2).unwrap(),Pos::from('B',3).unwrap()).unwrap();
    match game.move_(Pos::from('B',3).unwrap(),Pos::from('B',2).unwrap()) {
        Ok (_) => assert!(false),
        Err (e) => assert_eq!("Illegal move: Pawn cannot move in such a manner", e)
    }
}

#[test]
fn white_knight_can_move_in_l_shape() {
    let game = GameImpl::new().unwrap();
    let game = game.move_(Pos::from('G',1).unwrap(), Pos::from('F',3).unwrap()).unwrap();
    
    match game.get_piece(Pos::from('F',3).unwrap()) {
        None => assert!(false),
        Some(p) => assert_eq!(&Knight(White), p)
    }
}

#[test]
fn white_knight_cant_move_forward() {
    let game = GameImpl::new().unwrap();
    match game.move_(Pos::from('G',1).unwrap(), Pos::from('G',2).unwrap()) {
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
    let game = game.move_(Pos::from('D',5).unwrap(), Pos::from('B',7).unwrap()).unwrap();

    let p = game.get_piece(Pos::from('B',7).unwrap()).expect("Expected: white bishop\nGot: none");
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
    match game.move_(Pos::from('D',5).unwrap(), Pos::from('B',5).unwrap()) {
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
    let game = game.move_(Pos::from('D',5).unwrap(), Pos::from('D',8).unwrap()).unwrap();

    let p = game.get_piece(Pos::from('D',8).unwrap()).expect("Expected: white rook\nGot: none");
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
    let game = game.move_(Pos::from('D',5).unwrap(), Pos::from('B',7).unwrap()).unwrap();

    let p = game.get_piece(Pos::from('B',7).unwrap()).expect("Expected: white queen\nGot: none");
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
    let game = game.move_(Pos::from('D',5).unwrap(), Pos::from('D',2).unwrap()).unwrap();

    let p = game.get_piece(Pos::from('D',2).unwrap()).expect("Expected: white queen\nGot: none");
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
    match game.move_(Pos::from('D',5).unwrap(), Pos::from('E',8).unwrap()) {
        Err(e) => assert_eq!("Illegal move: Queen cannot move in such a manner",e),
        Ok(_) => panic!()
    }
}