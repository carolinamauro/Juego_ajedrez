use crate::{piece::Piece, position::Position};


pub trait NewBlackPiece {
    fn new(pos: Position) -> Piece;
}

pub trait NewWhitePiece {
    fn new(pos: Position) -> Piece;
}