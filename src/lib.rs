use board::get_chess_pieces;

pub mod board;
pub mod piece;
pub mod position;
pub fn get_move_result(file: &String){
    get_chess_pieces(file);

}