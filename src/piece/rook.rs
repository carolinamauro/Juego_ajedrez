use crate::position::Position;

#[derive(Debug)]
pub struct Rook;

impl Rook {
    pub fn capture_piece(rook_pos: &Position, pos_piece: &Position) -> bool {
        let check_horizontal = rook_pos.same_horizontal(pos_piece);
        let check_vertical = rook_pos.same_vertical(pos_piece);

        return check_horizontal || check_vertical;
    }
}

#[test]
fn test_piece_movements() {
    //Arriba
    assert!(Rook::capture_piece(
        &Position::new(7, 7),
        &Position::new(0, 7)
    ));
    //Abajo
    assert!(Rook::capture_piece(
        &Position::new(0, 5),
        &Position::new(3, 5)
    ));
    //Derecha
    assert!(Rook::capture_piece(
        &Position::new(4, 7),
        &Position::new(4, 2)
    ));
    //Izquierda
    assert!(Rook::capture_piece(
        &Position::new(7, 2),
        &Position::new(7, 7)
    ));
    //No capturable
    assert_eq!(
        false,
        Rook::capture_piece(&Position::new(7, 7), &Position::new(6, 3))
    );
}
