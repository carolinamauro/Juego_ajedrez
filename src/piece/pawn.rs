use crate::{
    piece::{Color, PieceData},
    position::Position,
};

#[derive(Debug)]
// Representación del Peón
pub struct Pawn;

// Devuelve si el pawn puede o no capturar a la pieza que se encuentra en pos_piece
// Si el peón es negro, se verifica si la posición de la pieza se encuentra inmediatamente
// debajo y en diagonal a la izquierda o derecha del peón.
// Si el peón es blanco, se verifica si la posición de la pieza se encuentra inmediatamente
// arriba y en diagonal a la izquierda o derecha del peón.
// Devuelve true si puede capturar y false en caso contrario
impl Pawn {
    pub fn capture_piece(pawn: &PieceData, pos_piece: &Position) -> bool {
        match pawn.0 {
            Color::Black => pawn.1.same_diagonal_immediately_below(pos_piece),
            Color::White => pawn.1.same_diagonal_immediately_above(pos_piece),
        }
    }
}

#[test]
fn test_piece_movement_black_piece() {
    let my_pawn = PieceData(Color::Black, Position::new(3, 3));
    assert!(Pawn::capture_piece(&my_pawn, &Position::new(4, 2)));
    assert!(Pawn::capture_piece(&my_pawn, &Position::new(4, 4)));
}

#[test]
fn test_piece_movement_white_piece() {
    let my_pawn = PieceData(Color::White, Position::new(6, 4));
    assert!(Pawn::capture_piece(&my_pawn, &Position::new(5, 3)));
    assert!(Pawn::capture_piece(&my_pawn, &Position::new(5, 5)));
}

#[test]
fn test_piece_movement_no_capturable() {
    let my_pawn = PieceData(Color::White, Position::new(6, 4));
    assert_eq!(false, Pawn::capture_piece(&my_pawn, &Position::new(0, 1)));
    let my_pawn = PieceData(Color::Black, Position::new(7, 0));
    assert_eq!(false, Pawn::capture_piece(&my_pawn, &Position::new(5, 3)));
}
