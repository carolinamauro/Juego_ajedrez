use crate::position::Position;

#[derive(Debug)]
pub struct Knight;


impl Knight {
    pub fn capture_piece(knight_pos: &Position, pos_piece: &Position) -> bool {
        let x_diff = (pos_piece.x - knight_pos.x).abs();
        let y_diff = (pos_piece.y - knight_pos.y).abs();
        return (x_diff == 1 && y_diff == 2) || (x_diff == 2 && y_diff == 1)
    }
}


#[test]
fn test_piece_movements() {
    //Arriba izquierda
    assert!(Knight::capture_piece(&Position::new(7, 2), &Position::new(5, 1)));
    assert!(Knight::capture_piece(&Position::new(7, 2), &Position::new(6, 0)));
    //Arriba derecha
    assert!(Knight::capture_piece(&Position::new(7, 2), &Position::new(5, 3)));
    assert!(Knight::capture_piece(&Position::new(7, 2), &Position::new(6, 4)));
    //Abajo izquieda
    assert!(Knight::capture_piece(&Position::new(0, 1), &Position::new(2, 0)));
    assert!(Knight::capture_piece(&Position::new(0, 6), &Position::new(1, 4)));
    //Abajo derecha
    assert!(Knight::capture_piece(&Position::new(0, 1), &Position::new(2, 2)));
    assert!(Knight::capture_piece(&Position::new(0, 5), &Position::new(1, 7)));
    //No captura
    assert_eq!(false, Knight::capture_piece(&Position::new(4, 0), &Position::new(1, 7)));
}
