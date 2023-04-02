use crate::{traits::{NewBlackPiece, NewWhitePiece}, position::Position};

use super::{Piece, Color, Pieces};

pub struct Rook {
    color: Color,
    pos: Position,
    //moviemientos posibles
}

impl NewBlackPiece for Rook {
    fn new(pos: Position) -> Piece {
        let color: Color = Color::Black;
        //agregar los movimientos posibles del rey
        return Piece { piece: (Pieces::Rook { chess_piece: (Rook { color, pos }) }) };
    }
}

impl NewWhitePiece for Rook {
    fn new(pos: Position) -> Piece {
        let color: Color = Color::White;
        //agregar los movimientos posibles del rey
        return Piece { piece: (Pieces::Rook { chess_piece: (Rook { color, pos }) }) };
    }
}