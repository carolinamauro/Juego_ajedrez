use crate::{traits::{NewBlackPiece, NewWhitePiece}, position::Position};
use super::{Piece, Color, Pieces};

pub struct Bishop {
    color: Color,
    pos: Position,
    //moviemientos posibles
}

impl NewBlackPiece for Bishop {
    fn new(pos: Position) -> Piece {
        let color: Color = Color::Black;
        //agregar los movimientos posibles del rey
        return Piece { piece: (Pieces::Bishop { chess_piece: (Bishop { color, pos }) }) };
    }
}

impl NewWhitePiece for Bishop {
    fn new(pos: Position) -> Piece {
        let color: Color = Color::White;
        //agregar los movimientos posibles del rey
        return Piece { piece: (Pieces::Bishop { chess_piece: (Bishop { color, pos }) }) };
    }
}