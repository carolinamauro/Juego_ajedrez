mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

use crate::constants::{
    BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, BLACK_WINS,
    LOSE_MOVE, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
    WHITE_WINS,
};
use crate::position::Position;

use self::bishop::Bishop;
use self::king::King;
use self::knight::Knight;
use self::pawn::Pawn;
use self::queen::Queen;
use self::rook::Rook;

// Posibles piezas del ajedrez, cada una con su respectiva información sobre su color y posición
// en el tablero
#[derive(Debug)]
pub enum Pieces {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

// Devuelve el resultado de la jugada en función si la pieza puede o no comer a la pieza que se enfrenta
// (result == true puede comer). Si puede y es una pieza blanca devuelve W, si es negra B y en caso de no poder P

fn result_capture_price(piece: &Piece, result: bool) -> char {
    if result {
        match piece.get_color() {
            Color::Black => return BLACK_WINS,
            Color::White => return WHITE_WINS,
        }
    }
    LOSE_MOVE
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
    pub fn get_piece(&self) -> &Pieces {
        &self.piece
    }

    // Dada la información de una pieza y el resultado de la partida que "jugaría" (si pudue no capturar a otra
    // pieza) se devuelve P(perdió), E(empató), B(ganaría la partida y la pieza es negra) o W(ganaría la partida y la pieza es blanca)

    pub fn capture_piece(piece: &Piece, piece_to_capture: &Piece) -> char {
        let pos_piece: Position = piece_to_capture.get_position();
        let result = match piece.get_piece() {
            Pieces::King => King::capture_piece(&piece.get_position(), &pos_piece),
            Pieces::Queen => Queen::capture_piece(&piece.get_position(), &pos_piece),
            Pieces::Bishop => Bishop::capture_piece(&piece.get_position(), &pos_piece),
            Pieces::Knight => Knight::capture_piece(&piece.get_position(), &pos_piece),
            Pieces::Pawn => Pawn::capture_piece(piece, &pos_piece),
            Pieces::Rook => Rook::capture_piece(&piece.get_position(), &pos_piece),
        };

        result_capture_price(piece, result)
    }
}

//agregar test
