use crate::{position::Position, piece::{Pieces, PieceData, Color, capture_piece}};

#[derive(Debug)]
pub struct Queen;

impl Queen {
    pub fn capture_piece(pos_queen: Position, pos_piece: Position) -> bool {
        let check_diagonals: bool = pos_queen.same_diagonal(pos_piece);
        let check_horizontal: bool = pos_queen.same_horizontal(pos_piece);
        let check_vertical: bool = pos_queen.same_vertical(pos_piece);

        return  check_diagonals || check_horizontal || check_vertical;
    }
}

#[test]
fn test_piece_movements() {
    //Diagonal 1
    let pos: Position = Position::new(1, 2);
    let my_queen: Pieces = Pieces::Queen(PieceData(Color::Black, Position::new(5, 6)));
    assert_eq!(true, capture_piece(my_queen, pos));
    //Diagonal 2
    let pos: Position = Position::new(6, 1);
    let my_queen: Pieces = Pieces::Queen(PieceData(Color::Black, Position::new(5, 2)));
    assert_eq!(true, capture_piece(my_queen, pos));
    //Arriba
    let pos: Position = Position::new(0, 7);
    let my_queen: Pieces = Pieces::Queen(PieceData(Color::White, Position::new(7, 7)));
    assert_eq!(true, capture_piece(my_queen, pos));
    //Abajo
    let pos: Position = Position::new(3, 5);
    let my_queen: Pieces = Pieces::Queen(PieceData(Color::Black, Position::new(0, 5)));
    assert_eq!(true, capture_piece(my_queen, pos));
    //Derecha 
    let pos: Position = Position::new(4, 2);
    let my_queen: Pieces = Pieces::Queen(PieceData(Color::White, Position::new(4, 7)));
    assert_eq!(true, capture_piece(my_queen, pos));
    //Izquierda 
    let pos: Position = Position::new(7, 7);
    let my_queen: Pieces = Pieces::Queen(PieceData(Color::Black, Position::new(7, 2)));
    assert_eq!(true, capture_piece(my_queen, pos));
    //No capturable
    let pos: Position = Position::new(6, 3);
    let my_queen: Pieces = Pieces::Queen(PieceData(Color::Black, Position::new(1, 5)));
    assert_eq!(false, capture_piece(my_queen, pos));
}