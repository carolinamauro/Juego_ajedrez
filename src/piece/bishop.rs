use crate::{traits::{NewBlackPiece, NewWhitePiece}, position::Position};
use super::{Piece, Color, Pieces, PieceData};

#[derive(Debug)]
pub struct Bishop {
    color: Color,
    pos: Position,
    //moviemientos posibles
}

impl NewBlackPiece for Bishop {
    fn new(pos: Position) -> Pieces {
        let color: Color = Color::Black;
        //agregar los movimientos posibles del rey
        return Pieces::Bishop(PieceData { color, pos });
    }
}

impl NewWhitePiece for Bishop {
    fn new(pos: Position) -> Pieces {
        let color: Color = Color::White;
        //agregar los movimientos posibles del rey
        return Pieces::Bishop(PieceData { color, pos });
    }
}



