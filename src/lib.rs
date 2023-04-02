use board::get_chess_pieces;
use piece::Pieces;
pub mod board;
pub mod piece;

pub mod position;
pub fn get_move_result(file: &String){
    let mut chess_pieces: (Option<Pieces>, Option<Pieces>);
   
    match get_chess_pieces(file)  {
        Ok(p) => chess_pieces = p,
        Err(err) => {
            panic!("ERROR: {:} al intentar obtener las piezas", err);
        }
    } 



    

}