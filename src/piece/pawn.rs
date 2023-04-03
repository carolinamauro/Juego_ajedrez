use crate::{position::Position, piece::{PieceData, Color}};

#[derive(Debug)]
pub struct Pawn;

impl Pawn {
    pub fn capture_piece(pawn: &PieceData, pos_piece: &Position) -> bool {
        match pawn.0 {
            Color::Black => return pawn.1.same_diagonal_immediately_below(pos_piece),
            Color::White => return pawn.1.same_diagonal_immediately_above(pos_piece),
        }
    }
}

#[test]
fn test_piece_movements() {
    //Pieza negra
    let my_pawn = PieceData(Color::Black, Position::new(3, 3));
    assert!(Pawn::capture_piece(&my_pawn, &Position::new(4, 2)));
    assert!(Pawn::capture_piece(&my_pawn, &Position::new(4, 4)));
    //Pieza blanca
    let my_pawn = PieceData(Color::White, Position::new(6, 4));
    assert!(Pawn::capture_piece(&my_pawn, &Position::new(5,3)));
    assert!(Pawn::capture_piece(&my_pawn, &Position::new(5, 5)));
    //No capturable
    let my_pawn = PieceData(Color::White, Position::new(6, 4));
    assert_eq!(false,Pawn::capture_piece(&my_pawn, &Position::new(0,1)));
    let my_pawn = PieceData(Color::Black, Position::new(7, 0));
    assert_eq!(false,Pawn::capture_piece(&my_pawn, &Position::new(5,3)));
}