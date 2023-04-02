use crate::{piece::Pieces, position::Position};


pub trait NewBlackPiece {
    fn new(pos: Position) -> Pieces;
}

pub trait NewWhitePiece {
    fn new(pos: Position) -> Pieces;
}

pub trait CapturePiece {
    fn capture_piece(pos: Position) -> String;
}