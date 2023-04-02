use crate::{position::Position, piece::capture_piece};

use super::{Pieces, PieceData, Color};

#[derive(Debug)]
pub struct King;

impl King {
    pub fn capture_piece(pos_king: Position, pos_piece: Position) -> bool {
        let check_diagonals: bool = pos_king.same_diagonal_immediately_above(pos_piece) || pos_king.same_diagonal_immediately_below(pos_piece);
        let check_horizontal: bool = (pos_king.x + 1) == pos_piece.x || (pos_king.x - 1) == pos_piece.x;
        let check_vertical: bool = (pos_king.y + 1) == pos_piece.y || (pos_king.y - 1) == pos_piece.y;

        return  check_diagonals || check_horizontal || check_vertical;
    }
}

#[test]
fn test_piece_movements() {
    //Diagonal
    let pos: Position = Position::new(1, 1);
    let my_king: Pieces = Pieces::King(PieceData(Color::Black, Position::new(0, 0)));
    assert_eq!(true, capture_piece(my_king, pos));
    //Arriba
    let pos: Position = Position::new(1, 1);
    let my_king: Pieces = Pieces::King(PieceData(Color::White, Position::new(0, 1)));
    assert_eq!(true, capture_piece(my_king, pos));
    //Abajo
    let pos: Position = Position::new(7, 5);
    let my_king: Pieces = Pieces::King(PieceData(Color::Black, Position::new(7, 6)));
    assert_eq!(true, capture_piece(my_king, pos));
    //Derecha 
    let pos: Position = Position::new(7, 3);
    let my_king: Pieces = Pieces::King(PieceData(Color::White, Position::new(7, 4)));
    assert_eq!(true, capture_piece(my_king, pos));
    //Izquierda 
    let pos: Position = Position::new(4, 7);
    let my_king: Pieces = Pieces::King(PieceData(Color::Black, Position::new(3, 7)));
    assert_eq!(true, capture_piece(my_king, pos));
    //No capturable
    let pos: Position = Position::new(5, 9);
    let my_king: Pieces = Pieces::King(PieceData(Color::Black, Position::new(0, 0)));
    assert_eq!(false, capture_piece(my_king, pos));
}