use crate::{traits::{NewBlackPiece, NewWhitePiece}, position::Position};

use super::{Piece, Color, Pieces};

pub struct Queen {
    color: Color,
    pos: Position,
}


impl NewBlackPiece for Queen {
    fn new(pos: Position) -> Piece {
        let color: Color = Color::Black;  
        //agregar los movimientos posibles del rey
        return Piece { piece: (Pieces::Queen { chess_piece: (Queen { color, pos }) }) };
    }
}

impl NewWhitePiece for Queen {
    fn new(pos: Position) -> Piece {
        let color: Color = Color::White;
        //agregar los movimientos posibles del rey
        return Piece { piece: (Pieces::Queen { chess_piece: (Queen { color, pos }) }) };
    }
}
