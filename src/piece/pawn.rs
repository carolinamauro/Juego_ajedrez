use crate::{traits::{NewBlackPiece, NewWhitePiece}, position::Position};

use super::{Piece, Color, Pieces};


pub struct Pawn {
    color: Color,
    pos: Position,
    //moviemientos posibles
}


impl NewBlackPiece for Pawn {
    fn new(pos: Position) -> Piece {
        let color: Color = Color::Black;
        //agregar los movimientos posibles del rey
        return Piece { piece: (Pieces::Pawn { chess_piece: (Pawn {color, pos }) }) };
    }
}

impl NewWhitePiece for Pawn {
    fn new(pos: Position) -> Piece {
        let color: Color = Color::White;
        //agregar los movimientos posibles del rey
        return Piece { piece: (Pieces::Pawn { chess_piece: (Pawn {color, pos }) }) };
    }
}