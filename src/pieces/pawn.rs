use crate::{
    pieces::{Color, Piece},
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
    pub fn capture_piece(pawn: &Piece, pos_piece: &Position) -> bool {
        match pawn.get_color() {
            Color::Black => pawn
                .get_position()
                .same_diagonal_immediately_below(pos_piece),
            Color::White => pawn
                .get_position()
                .same_diagonal_immediately_above(pos_piece),
        }
    }
}

#[test]
fn test_piece_movement_black_piece() {
    let my_pawn = match Piece::new('P', Position::new(3, 3)) {
        Some(p) => p,
        None => {
            assert!(false);
            return;
        }
    };

    assert!(Pawn::capture_piece(&my_pawn, &Position::new(4, 2)));
    assert!(Pawn::capture_piece(&my_pawn, &Position::new(4, 4)));
}

#[test]
fn test_piece_movement_white_piece() {
    let my_pawn = match Piece::new('p', Position::new(6, 4)) {
        Some(p) => p,
        None => {
            assert!(false);
            return;
        }
    };

    assert!(Pawn::capture_piece(&my_pawn, &Position::new(5, 3)));
    assert!(Pawn::capture_piece(&my_pawn, &Position::new(5, 5)));
}

#[test]
fn test_white_pawn_no_capturable() {
    let my_pawn = match Piece::new('p', Position::new(6, 4)) {
        Some(p) => p,
        None => {
            assert!(false);
            return;
        }
    };
    assert_eq!(false, Pawn::capture_piece(&my_pawn, &Position::new(0, 1)));
}

#[test]
fn test_black_pawn_no_capturable() {
    let my_pawn = match Piece::new('P', Position::new(6, 4)) {
        Some(p) => p,
        None => {
            assert!(false);
            return;
        }
    };
    assert_eq!(false, Pawn::capture_piece(&my_pawn, &Position::new(5, 3)));
}
