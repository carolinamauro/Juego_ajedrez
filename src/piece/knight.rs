use crate::{traits::{NewBlackPiece, NewWhitePiece}, position::Position};

use super::{Color, Pieces, PieceData};

#[derive(Debug)]
pub struct Knight;

impl NewBlackPiece for Knight {
    fn new(pos: Position) -> Pieces {
        let color: Color = Color::Black;
        return Pieces::Knight(PieceData {color, pos});
    }
}

impl NewWhitePiece for Knight {
    fn new(pos: Position) -> Pieces {
        let color: Color = Color::White;
        return Pieces::Knight(PieceData {color, pos});
    }
}