use crate::position::Position;

#[derive(Debug)]
pub struct Queen;

impl Queen {
    pub fn capture_piece(pos_queen: &Position, pos_piece: &Position) -> bool {
        let check_diagonals: bool = pos_queen.same_diagonal(pos_piece);
        let check_horizontal: bool = pos_queen.same_horizontal(pos_piece);
        let check_vertical: bool = pos_queen.same_vertical(pos_piece);

        return  check_diagonals || check_horizontal || check_vertical;
    }
}

#[test]
fn test_piece_movements() {
    //Diagonal 1
    assert!(Queen::capture_piece(&Position::new(5, 6), &Position::new(1, 2)));
    //Diagonal 2
    assert!(Queen::capture_piece(&Position::new(5, 2), &Position::new(6, 1)));
    //Arriba
    assert!(Queen::capture_piece(&Position::new(7, 7), &Position::new(0, 7)));
    //Abajo
    assert!(Queen::capture_piece(&Position::new(3, 5), &Position::new(0, 5)));
    //Derecha 
    assert!(Queen::capture_piece(&Position::new(4, 7), &Position::new(4, 2)));
    //Izquierda 
    assert!(Queen::capture_piece(&Position::new(7, 2), &Position::new(7, 7)));
    //No capturable
    assert_eq!(false, Queen::capture_piece(&Position::new(1, 5), &Position::new(6, 3)));
}