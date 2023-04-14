use crate::position::Position;

use super::{
    bishop::Bishop,
    king::{King},
    knight::Knight,
    pawn::Pawn,
    queen::Queen,
    rook::Rook,
    Pieces,
};
#[derive(Clone, Copy, Debug)]
pub enum Color {
    Black,
    White,
}

// Estructura que se utiliza para guardar información de cada pieza del ajedrez
#[derive(Debug)]
pub struct Piece {
    piece: Pieces,
    color: Color,
    position: Position,
}

impl Piece {
    pub fn new(piece_type: char, position: Position) -> Option<Piece> {
        match piece_type {
            BLACK_KING => Some(Piece {
                piece: Pieces::King,
                color: Color::Black,
                position,
            }),
            WHITE_KING => Some(Piece {
                piece: Pieces::King,
                color: Color::White,
                position,
            }),
            BLACK_QUEEN => Some(Piece {
                piece: Pieces::Queen,
                color: Color::Black,
                position,
            }),
            WHITE_QUEEN => Some(Piece {
                piece: Pieces::Queen,
                color: Color::White,
                position,
            }),
            BLACK_BISHOP => Some(Piece {
                piece: Pieces::Bishop,
                color: Color::Black,
                position,
            }),
            WHITE_BISHOP => Some(Piece {
                piece: Pieces::Bishop,
                color: Color::White,
                position,
            }),
            BLACK_KNIGHT => Some(Piece {
                piece: Pieces::Knight,
                color: Color::Black,
                position,
            }),
            WHITE_KNIGHT => Some(Piece {
                piece: Pieces::Knight,
                color: Color::White,
                position,
            }),
            BLACK_ROOK => Some(Piece {
                piece: Pieces::Rook,
                color: Color::Black,
                position,
            }),
            WHITE_ROOK => Some(Piece {
                piece: Pieces::Rook,
                color: Color::White,
                position,
            }),
            BLACK_PAWN => Some(Piece {
                piece: Pieces::Pawn,
                color: Color::Black,
                position,
            }),
            WHITE_PAWN => Some(Piece {
                piece: Pieces::Pawn,
                color: Color::White,
                position,
            }),
            _ => None,
        }
    }
    pub fn get_color(&self) -> Color {
        self.color
    }
    pub fn get_position(&self) -> Position {
        self.position
    }
    pub fn get_piece(&self) -> Pieces {
        self.piece
    }

    pub fn capture_piece(piece: &Piece, pos_piece: &Position) -> bool {
        match piece.get_piece() {
            Pieces::King => King::capture_piece(&piece.get_position(), pos_piece),
            Pieces::Queen => Queen::capture_piece(&piece.get_position(), pos_piece),
            Pieces::Bishop => Bishop::capture_piece(&piece.get_position(), pos_piece),
            Pieces::Knight => Knight::capture_piece(&piece.get_position(), pos_piece),
            Pieces::Pawn => Pawn::capture_piece(piece, pos_piece),
            Pieces::Rook => Rook::capture_piece(&piece.get_position(), pos_piece),
        }
    }
}

//agregar test