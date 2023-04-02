use crate::{traits::{NewBlackPiece, NewWhitePiece}, position::Position};
use super::{Color, Pieces, PieceData};

#[derive(Debug)]
pub struct Pawn;


impl NewBlackPiece for Pawn {
    fn new(pos: Position) -> Pieces {
        let color: Color = Color::Black;
        return Pieces::Pawn(PieceData{color, pos });
    }
}

impl NewWhitePiece for Pawn {
    fn new(pos: Position) -> Pieces {
        let color: Color = Color::White;
        return Pieces::Pawn(PieceData{color, pos });
    }
}