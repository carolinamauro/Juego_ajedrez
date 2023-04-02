use crate::{traits::{NewBlackPiece, NewWhitePiece}, position::Position};

use super::{Color, Pieces, PieceData};

#[derive(Debug)]
pub struct Rook {
    color: Color,
    pos: Position,
}

impl NewBlackPiece for Rook {
    fn new(pos: Position) -> Pieces {
        let color: Color = Color::Black;
        return Pieces::Rook(PieceData{ color, pos });
    }
}

impl NewWhitePiece for Rook {
    fn new(pos: Position) -> Pieces {
        let color: Color = Color::White;
        return Pieces::Rook(PieceData{ color, pos });
    }
}