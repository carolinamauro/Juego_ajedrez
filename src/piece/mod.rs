mod king;
mod queen;
mod bishop;
mod rook;
mod pawn;
mod knight;

use crate::position::Position;

use self::{king::King, queen::Queen, bishop::Bishop, knight::Knight, rook::Rook, pawn::Pawn};

#[derive(Debug)]
pub struct PieceData(Color, Position);

#[derive(Debug)]
pub enum Pieces {
    King(PieceData), 
    Queen(PieceData),
    Bishop(PieceData), 
    Knight(PieceData),
    Rook(PieceData),
    Pawn(PieceData), 
}
// #[derive(Debug)]
// pub struct Piece;

#[derive(Debug)]
enum Color {
    Black,
    White,
}

impl Pieces {
    pub fn new(piece_type: char, pos: Position) -> Option<Pieces> {
        match piece_type {
            'R' => Some(Pieces::King(PieceData(Color::Black, pos))),
            'r' => Some(Pieces::King(PieceData(Color::White, pos))),
            'D' => Some(Pieces::Queen(PieceData(Color::Black, pos))),
            'd' => Some(Pieces::Queen(PieceData(Color::White, pos))),
            'A' => Some(Pieces::Bishop(PieceData(Color::Black, pos))),
            'a' => Some(Pieces::Bishop(PieceData(Color::White, pos))),
            'C' => Some(Pieces::Knight(PieceData(Color::Black, pos))),
            'c' => Some(Pieces::Knight(PieceData(Color::White, pos))),
            'T' => Some(Pieces::Rook(PieceData(Color::Black, pos))),
            't' => Some(Pieces::Rook(PieceData(Color::White, pos))),
            'P' => Some(Pieces::Pawn(PieceData(Color::Black, pos))),
            'p' => Some(Pieces::Pawn(PieceData(Color::White, pos))),
            _ => None,
        }
    }
}

pub fn capture_piece(piece: Pieces, pos_piece: Position) -> bool {
    match piece {
        Pieces::King(state) => King::capture_piece(&state.1, &pos_piece),
        Pieces::Queen(state) => Queen::capture_piece(&state.1, &pos_piece),
        Pieces::Bishop(state) => Bishop::capture_piece(&state.1, &pos_piece),
        Pieces::Knight(state) => Knight::capture_piece(&state.1, &pos_piece),
        Pieces::Rook(state) => Rook::capture_piece(&state.1, &pos_piece),
        Pieces::Pawn(state) => Pawn::capture_piece(&state, &pos_piece),
    }
}

#[test]
fn test_piece_creation() {
    let mut pos: Position = Position::new(0, 0);
    let mut is_none: bool = false;
 
    match Pieces::new('z', pos) {
        Some(p) => is_none = false,
        None => is_none = true,
    }
 
    assert!(is_none);

}