use crate::{traits::{NewBlackPiece, NewWhitePiece}, position::Position};

use super::{Piece, Color, Pieces};

pub struct King {
    color: Color,
    pos: Position,
    //moviemientos posibles
}


//lo hago separado porque tienen distinto comportamiento segun el color (los movimientos)
impl NewBlackPiece for King {

    fn new(pos: Position) -> Piece {
        let color: Color = Color::Black;
        //agregar los movimientos posibles del rey
        return Piece { piece: (Pieces::King { chess_piece: (King { color, pos }) }) };
    }
}

impl NewWhitePiece for King { 
    fn new(pos: Position) -> Piece {
        let color: Color = Color::White;
        //agregar los movimientos posibles del rey
        return Piece { piece: (Pieces::King { chess_piece: (King { color, pos }) }) };
    }
}