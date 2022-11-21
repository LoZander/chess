use std::ops::Sub;
use crate::framework::Piece::*;
use crate::framework::Player::*;

pub trait Game {
    type Game;
    fn move_(self, from: Pos, to: Pos) -> Result<Self::Game,String>;
    fn get_piece(&self, position: Pos) -> Option<&Piece>;
}

pub trait Gui {
    fn run(&self);
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Player {
    White,
    Black,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Piece {
    Pawn(Player),
    Rook(Player),
    Knight(Player),
    Bishop(Player),
    King(Player),
    Queen(Player)
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


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Clone, Copy)]
pub struct Pos(pub i8, pub i8);

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match self.0 {
            1 => 'A',
            2 => 'B',
            3 => 'C',
            4 => 'D',
            5 => 'E',
            6 => 'F',
            7 => 'G',
            8 => 'H',
            _ => '?'
        };
        write!(f, "{}{}", c, self.1)
    }
}

impl Sub for Pos {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Pos {
    pub fn from(x: char,y: i8) -> Result<Self,String> {
        let z = match x {
            'A' => Ok(1),
            'B' => Ok(2),
            'C' => Ok(3),
            'D' => Ok(4),
            'E' => Ok(5),
            'F' => Ok(6),
            'G' => Ok(7),
            'H' => Ok(8),
            _ => Err("Error: illegal pos value")
        }?;

        Ok(Pos(z,y))
    }
}