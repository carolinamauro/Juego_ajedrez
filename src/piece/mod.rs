mod king;
mod queen;
mod bishop;
mod rook;
mod pawn;
mod knight;

use crate::{position::Position, traits::{NewBlackPiece, NewWhitePiece}};
use crate::piece::{queen::Queen, king::King, bishop::Bishop, rook::Rook, pawn::Pawn, knight::Knight};
#[derive(Debug)]

pub struct  PieceData {
    color: Color,
    pos: Position,
}

pub enum Pieces {
    King(PieceData), 
    Queen(PieceData),
    Bishop(PieceData), 
    Knight(PieceData),
    Rook(PieceData),
    Pawn(PieceData), 
}
#[derive(Debug)]
pub struct Piece;

#[derive(Debug)]
enum Color {
    Black,
    White,
}

impl Piece {
    pub fn new(piece_type: char, pos: Position) -> Option<Pieces> {
        match piece_type {
            'R' => Some(<King as NewBlackPiece>::new(pos)),
            'r' => Some(<King as NewWhitePiece>::new(pos)),
            'D' => Some(<Queen as NewBlackPiece>::new(pos)),
            'd' => Some(<Queen as NewWhitePiece>::new(pos)),
            'A' => Some(<Bishop as NewBlackPiece>::new(pos)),
            'a' => Some(<Bishop as NewWhitePiece>::new(pos)),
            'C' => Some(<Knight as NewBlackPiece>::new(pos)),
            'c' => Some(<Knight as NewWhitePiece>::new(pos)),
            'T' => Some(<Rook as NewBlackPiece>::new(pos)),
            't' => Some(<Rook as NewWhitePiece>::new(pos)),
            'P' => Some(<Pawn as NewBlackPiece>::new(pos)),
            'p' => Some(<Pawn as NewWhitePiece>::new(pos)),
            _ => None,
        }
    }
  
}

#[test]
fn test_piece_creation() {
    let mut pos: Position = Position::new(0, 0);
    let mut is_none: bool = false;
 
    match Piece::new('z', pos) {
        Some(p) => is_none = false,
        None => is_none = true,
    }
 
    assert!(is_none);

}