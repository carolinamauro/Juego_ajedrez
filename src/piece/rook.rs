use crate::position::Position;

#[derive(Debug)]
// Representación de la Torre
pub struct Rook;
// Verifica si la pieza en pos_piece está en la fila o columna que rook_pos (Torre)
// devolviendo true si es así (torre puede capturar a la pieza en esa posición).
// En caso contrario devuelve false
impl Rook {
    pub fn capture_piece(rook_pos: &Position, pos_piece: &Position) -> bool {
        let check_horizontal = rook_pos.same_horizontal(pos_piece);
        let check_vertical = rook_pos.same_vertical(pos_piece);

        check_horizontal || check_vertical
    }
}

#[test]
fn test_piece_movement_up() {
    assert!(Rook::capture_piece(
        &Position::new(7, 7),
        &Position::new(0, 7)
    ));
}

#[test]
fn test_piece_movement_down() {
    assert!(Rook::capture_piece(
        &Position::new(0, 5),
        &Position::new(3, 5)
    ));
}
#[test]
fn test_piece_movement_right() {
    assert!(Rook::capture_piece(
        &Position::new(4, 7),
        &Position::new(4, 2)
    ));
}

#[test]
fn test_piece_movement_left() {
    assert!(Rook::capture_piece(
        &Position::new(7, 2),
        &Position::new(7, 7)
    ));
}

#[test]
fn test_piece_movement_no_capturable() {
    assert_eq!(
        false,
        Rook::capture_piece(&Position::new(7, 7), &Position::new(6, 3))
    )
}
