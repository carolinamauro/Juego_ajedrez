use crate::{traits::{NewBlackPiece, NewWhitePiece}, position::Position};

use super::{Color, Pieces, PieceData};

#[derive(Debug)]
pub struct Queen;

impl NewBlackPiece for Queen {
    fn new(pos: Position) -> Pieces {
        let color: Color = Color::Black;  
        //agregar los movimientos posibles del rey
        return Pieces::Queen(PieceData { color, pos });
    }
}

impl NewWhitePiece for Queen {
    fn new(pos: Position) -> Pieces {
        let color: Color = Color::White;
        return Pieces::Queen(PieceData { color, pos });
    }
}
