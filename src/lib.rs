use board::get_chess_pieces;
use piece::Piece;
pub mod traits;
pub mod board;
pub mod piece;

pub mod position;
pub fn get_move_result(file: &String){
    let mut chess_pieces: (Piece, Piece);
    match get_chess_pieces(file)  {
        Ok(p) => chess_pieces = p,
        Err(err) => {
            panic!("ERROR: {:} al intentar obtener las piezas", err);
        }
    } 

    //YA TENGO DOS PIEZAS VALIDAS PUEDO JUGAR


}