mod bishop;
mod king;
mod knight;
mod pawn;
mod piece_data;
mod queen;
mod rook;

use crate::constants::{
    BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, BLACK_WINS,
    LOSE_MOVE, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
    WHITE_WINS,
};
use crate::position::Position;

use self::{
    bishop::Bishop,
    king::King,
    knight::Knight,
    pawn::Pawn,
    piece_data::{Color, PieceData},
    queen::Queen,
    rook::Rook,
};

// Posibles piezas del ajedrez, cada una con su respectiva información sobre su color y posición
// en el tablero
#[derive(Debug)]
pub enum Pieces {
    King(PieceData),
    Queen(PieceData),
    Bishop(PieceData),
    Knight(PieceData),
    Rook(PieceData),
    Pawn(PieceData),
}

// Función utilizada para obtener la pieza que corresponde al char leído en el archivo con el que
// estamos trabajando (contenido en el campo piece_char).
// Se toman las posibles piezas Rey [R], Dama [D], Alfil [A], Caballo [C], Torre [T], Peon [P] tanto en mayuscula
// como en minuscula y se obtiene la Piece que corresponda correctamente inicializada
// En caso que no exista una Pieza que corresponda con el piece_type solicitado, se devuelve None
impl Pieces {
    pub fn new(piece_type: char, pos: Position) -> Option<Pieces> {
        match piece_type {
            BLACK_KING => Some(Pieces::King(PieceData::new(Color::Black, pos))),
            WHITE_KING => Some(Pieces::King(PieceData::new(Color::White, pos))),
            BLACK_QUEEN => Some(Pieces::Queen(PieceData::new(Color::Black, pos))),
            WHITE_QUEEN => Some(Pieces::Queen(PieceData::new(Color::White, pos))),
            BLACK_BISHOP => Some(Pieces::Bishop(PieceData::new(Color::Black, pos))),
            WHITE_BISHOP => Some(Pieces::Bishop(PieceData::new(Color::White, pos))),
            BLACK_KNIGHT => Some(Pieces::Knight(PieceData::new(Color::Black, pos))),
            WHITE_KNIGHT => Some(Pieces::Knight(PieceData::new(Color::White, pos))),
            BLACK_ROOK => Some(Pieces::Rook(PieceData::new(Color::Black, pos))),
            WHITE_ROOK => Some(Pieces::Rook(PieceData::new(Color::White, pos))),
            BLACK_PAWN => Some(Pieces::Pawn(PieceData::new(Color::Black, pos))),
            WHITE_PAWN => Some(Pieces::Pawn(PieceData::new(Color::White, pos))),
            _ => None,
        }
    }
}

fn get_position(piece: &Pieces) -> Position {
    match piece {
        Pieces::King(state) => state.get_position(),
        Pieces::Queen(state) => state.get_position(),
        Pieces::Bishop(state) => state.get_position(),
        Pieces::Knight(state) => state.get_position(),
        Pieces::Rook(state) => state.get_position(),
        Pieces::Pawn(state) => state.get_position(),
    }
}

// Dada la información de una pieza y el resultado de la partida que "jugaría" (si pudue no capturar a otra
// pieza) se devuelve P(perdió), E(empató), B(ganaría la partida y la pieza es negra) o W(ganaría la partida y la pieza es blanca)
fn check_winner(piece_data: &PieceData, result: bool) -> char {
    if !result {
        return LOSE_MOVE;
    }
    match piece_data.get_color() {
        Color::Black => {
            if result {
                BLACK_WINS
            } else {
                LOSE_MOVE
            }
        }
        Color::White => {
            if result {
                WHITE_WINS
            } else {
                LOSE_MOVE
            }
        }
    }
}

// Devuelve el resultado de enfrentar a la pieza con otra. Se toma a la piece como referente y se devuelve
// P E B o W según corresponda, en función si puede o no capturar a la piece_to_capture.
pub fn can_capture_piece(piece: &Pieces, piece_to_capture: &Pieces) -> char {
    let pos_piece: Position = get_position(piece_to_capture);
    match piece {
        Pieces::King(state) => check_winner(
            state,
            King::capture_piece(&state.get_position(), &pos_piece),
        ),
        Pieces::Queen(state) => check_winner(
            state,
            Queen::capture_piece(&state.get_position(), &pos_piece),
        ),
        Pieces::Bishop(state) => check_winner(
            state,
            Bishop::capture_piece(&state.get_position(), &pos_piece),
        ),
        Pieces::Knight(state) => check_winner(
            state,
            Knight::capture_piece(&state.get_position(), &pos_piece),
        ),
        Pieces::Rook(state) => check_winner(
            state,
            Rook::capture_piece(&state.get_position(), &pos_piece),
        ),
        Pieces::Pawn(state) => check_winner(state, Pawn::capture_piece(state, &pos_piece)),
    }
}

#[test]
fn test_capture_black_wins() {
    assert_eq!(
        BLACK_WINS,
        can_capture_piece(
            &Pieces::Queen(PieceData::new(Color::Black, Position::new(0, 0))),
            &Pieces::Rook(PieceData::new(Color::White, Position::new(0, 7)))
        )
    );
}

#[test]
fn test_capture_white_wins() {
    assert_eq!(
        WHITE_WINS,
        can_capture_piece(
            &Pieces::Rook(PieceData::new(Color::White, Position::new(0, 7))),
            &Pieces::Queen(PieceData::new(Color::Black, Position::new(0, 0)))
        )
    );
}

#[test]
fn test_noone_caputes() {
    assert_eq!(
        LOSE_MOVE,
        can_capture_piece(
            &Pieces::Pawn(PieceData::new(Color::White, Position::new(5, 7))),
            &Pieces::Knight(PieceData::new(Color::Black, Position::new(2, 2)))
        )
    );
}
