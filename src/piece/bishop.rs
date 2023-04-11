use crate::position::Position;

#[derive(Debug)]
// Representación del Alfíl
pub struct Bishop;

// Verifica si la pieza en pos_piece está en la misma diagonal, ya sea por izquierda o derecha, que
// bischop_pos devolviendo true si es así (alfíl puede capturar a la pieza en esa posición).
// En caso contrario devuelve false
impl Bishop {
    pub fn capture_piece(bishop_pos: &Position, pos_piece: &Position) -> bool {
        bishop_pos.same_diagonal(pos_piece)
    }
}

#[test]
fn test_piece_movement_diagonal_left() {
    assert!(Bishop::capture_piece(
        &Position::new(5, 6),
        &Position::new(1, 2)
    ));
}
#[test]
fn test_piece_movement_diagonal_right() {
    assert!(Bishop::capture_piece(
        &Position::new(5, 2),
        &Position::new(6, 1)
    ));
}

#[test]
fn test_piece_movement_no_capturable() {
    assert_eq!(
        false,
        Bishop::capture_piece(&Position::new(6, 6), &Position::new(6, 5))
    );
}
