use crate::position::Position;

#[derive(Debug)]
// Representación de la Reina
pub struct Queen;

// Verifica si la pieza en pos_piece está en la misma diagonal, fila o columna que la reina en queen_pos,
// devolviendo true si es así (reina puede capturar a la pieza en esa posición).
// En caso contrario devuelve false
impl Queen {
    pub fn capture_piece(queen_pos: &Position, pos_piece: &Position) -> bool {
        let check_diagonals: bool = queen_pos.same_diagonal(pos_piece);
        let check_horizontal: bool = queen_pos.same_horizontal(pos_piece);
        let check_vertical: bool = queen_pos.same_vertical(pos_piece);

        check_diagonals || check_horizontal || check_vertical
    }
}

#[test]
fn test_piece_movement_diagolal_left() {
    assert!(Queen::capture_piece(
        &Position::new(5, 6),
        &Position::new(1, 2)
    ));
}

#[test]
fn test_piece_movement_diagonal_right() {
    assert!(Queen::capture_piece(
        &Position::new(5, 2),
        &Position::new(6, 1)
    ));
}

#[test]
fn test_piece_movement_up() {
    assert!(Queen::capture_piece(
        &Position::new(7, 7),
        &Position::new(0, 7)
    ));
}
#[test]
fn test_piece_movement_down() {
    assert!(Queen::capture_piece(
        &Position::new(3, 5),
        &Position::new(0, 5)
    ));
}

#[test]
fn test_piece_movement_right() {
    assert!(Queen::capture_piece(
        &Position::new(4, 7),
        &Position::new(4, 2)
    ));
}

#[test]
fn test_piece_movement_left() {
    assert!(Queen::capture_piece(
        &Position::new(7, 2),
        &Position::new(7, 7)
    ));
}

#[test]
fn test_piece_movement_no_capturable() {
    assert_eq!(
        false,
        Queen::capture_piece(&Position::new(1, 5), &Position::new(6, 3))
    );
}
