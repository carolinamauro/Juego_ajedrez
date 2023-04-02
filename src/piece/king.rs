use crate::{traits::{NewBlackPiece, NewWhitePiece, CapturePiece}, position::Position};

use super::{Color, Pieces, PieceData};
#[derive(Debug)]
pub struct King;

//lo hago separado porque tienen distinto comportamiento segun el color (los movimientos)
impl NewBlackPiece for King {
    fn new(pos: Position) -> Pieces {
        let color: Color = Color::Black;
        return Pieces::King(PieceData{color, pos});
    }
}

impl NewWhitePiece for King { 
    fn new(pos: Position) -> Pieces {
        let color: Color = Color::White;
        return Pieces::King(PieceData{color, pos});
    }
}

// impl CapturePiece for King {
//     fn capture_piece(pos: Position) -> String {
        
//     }
// }