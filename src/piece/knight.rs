use crate::{traits::{NewBlackPiece, NewWhitePiece}, position::Position};

use super::{Piece, Color, Pieces};


pub struct Knight {
    color: Color,
    pos: Position,
    //moviemientos posibles
}


impl NewBlackPiece for Knight {
    fn new(pos: Position) -> Piece {
        let color: Color = Color::Black;
        //agregar los movimientos posibles del rey
        return Piece { piece: (Pieces::Knight { chess_piece: (Knight {color, pos}) }) };
    }
}

impl NewWhitePiece for Knight {
    fn new(pos: Position) -> Piece {
        let color: Color = Color::White;
        //agregar los movimientos posibles del rey
        return Piece { piece: (Pieces::Knight { chess_piece: (Knight { color, pos }) }) };
    }
}