mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

use crate::position::Position;

use self::{bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook};

// Estructura que se utiliza para guardar información de cada pieza del ajedrez
#[derive(Debug)]
pub struct PieceData(Color, Position);

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

#[derive(Debug)]
enum Color {
    Black,
    White,
}

// Función utilizada para obtener la pieza que corresponde al char leído en el archivo con el que
// estamos trabajando (contenido en el campo piece_char).
// Se toman las posibles piezas Rey [R], Dama [D], Alfil [A], Caballo [C], Torre [T], Peon [P] tanto en mayuscula
// como en minuscula y se obtiene la Piece que corresponda correctamente inicializada
// En caso que no exista una Pieza que corresponda con el piece_type solicitado, se devuelve None
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

fn get_position(piece: &Pieces) -> Position {
    match piece {
        Pieces::King(state) => state.1,
        Pieces::Queen(state) => state.1,
        Pieces::Bishop(state) => state.1,
        Pieces::Knight(state) => state.1,
        Pieces::Rook(state) => state.1,
        Pieces::Pawn(state) => state.1,
    }
}

// Dada la información de una pieza y el resultado de la partida que "jugaría" (si pudue no capturar a otra
// pieza) se devuelve P(perdió), E(empató), B(ganaría la partida y la pieza es negra) o W(ganaría la partida y la pieza es blanca) 
fn check_winner(piece_data: &PieceData, result: bool) -> char {
    if !result {
        return 'P';
    }
    match piece_data.0 {
        Color::Black => {
            if result {
                return 'B';
            } else {
                return 'P';
            }
        }
        Color::White => {
            if result {
                return 'W';
            } else {
                return 'P';
            }
        }
    }
}

// Devuelve el resultado de enfrentar a la pieza con otra. Se toma a la piece como referente y se devuelve
// P E B o W según corresponda, en función si puede o no capturar a la piece_to_capture.
pub fn can_capture_piece(piece: &Pieces, piece_to_capture: &Pieces) -> char {
    let pos_piece: Position = get_position(&piece_to_capture);
    match piece {
        Pieces::King(state) => check_winner(state, King::capture_piece(&state.1, &pos_piece)),
        Pieces::Queen(state) => check_winner(state, Queen::capture_piece(&state.1, &pos_piece)),
        Pieces::Bishop(state) => check_winner(state, Bishop::capture_piece(&state.1, &pos_piece)),
        Pieces::Knight(state) => check_winner(state, Knight::capture_piece(&state.1, &pos_piece)),
        Pieces::Rook(state) => check_winner(state, Rook::capture_piece(&state.1, &pos_piece)),
        Pieces::Pawn(state) => check_winner(state, Pawn::capture_piece(&state, &pos_piece)),
    }
}

#[test]
fn test_capture() {
    assert_eq!(
        'B',
        can_capture_piece(
            &Pieces::Queen(PieceData(Color::Black, Position::new(0, 0))),
            &Pieces::Rook(PieceData(Color::White, Position::new(0, 7)))
        )
    );
    assert_eq!(
        'W',
        can_capture_piece(
            &Pieces::Rook(PieceData(Color::White, Position::new(0, 7))),
            &Pieces::Queen(PieceData(Color::Black, Position::new(0, 0)))
        )
    );
    assert_eq!(
        'P',
        can_capture_piece(
            &Pieces::Pawn(PieceData(Color::White, Position::new(5, 7))),
            &Pieces::Knight(PieceData(Color::Black, Position::new(2, 2)))
        )
    );
}
