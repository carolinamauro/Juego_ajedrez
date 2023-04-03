use crate::position::Position;

#[derive(Debug)]
pub struct Bishop;

impl Bishop {
    pub fn capture_piece(pos_bishop: &Position, pos_piece: &Position) -> bool {
        return  pos_bishop.same_diagonal(pos_piece);
    }
}

#[test]
fn test_piece_movements() {
    //Diagonal \
    assert!(Bishop::capture_piece(&Position::new(5, 6), &Position::new(1, 2)));
    //Diagonal /
    assert!(Bishop::capture_piece(&Position::new(5, 2), &Position::new(6, 1)));
    //No capturable
    assert_eq!(false, Bishop::capture_piece(&Position::new(6, 6), &Position::new(6, 5)));
}