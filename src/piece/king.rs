use crate::position::Position;

#[derive(Debug)]
pub struct King;

impl King {
    pub fn capture_piece(king_pos: &Position, pos_piece: &Position) -> bool {
        let check_diagonals: bool = king_pos.same_diagonal_immediately_above(pos_piece)
            || king_pos.same_diagonal_immediately_below(pos_piece);
        let check_horizontal: bool =
            (king_pos.x + 1) == pos_piece.x || (king_pos.x - 1) == pos_piece.x;
        let check_vertical: bool =
            (king_pos.y + 1) == pos_piece.y || (king_pos.y - 1) == pos_piece.y;

        return check_diagonals || check_horizontal || check_vertical;
    }
}

#[test]
fn test_piece_movements() {
    //Diagonal
    assert!(King::capture_piece(
        &Position::new(0, 0),
        &Position::new(1, 1)
    ));
    //Arriba
    assert!(King::capture_piece(
        &Position::new(0, 1),
        &Position::new(1, 1)
    ));
    //Abajo
    assert!(King::capture_piece(
        &Position::new(7, 6),
        &Position::new(7, 5)
    ));
    //Derecha
    assert!(King::capture_piece(
        &Position::new(7, 4),
        &Position::new(7, 3)
    ));
    //Izquierda
    assert!(King::capture_piece(
        &Position::new(3, 4),
        &Position::new(3, 3)
    ));
    //No capturable
    assert_eq!(
        false,
        King::capture_piece(&Position::new(0, 0), &Position::new(5, 7))
    );
}
