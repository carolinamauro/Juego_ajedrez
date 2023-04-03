use board::{get_chess_pieces, get_play_result};
pub mod board;
pub mod piece;

pub mod position;
pub fn print_move_result(file: &String) {
    let pieces = match get_chess_pieces(file) {
        Ok(p) => p,
        Err(err) => {
            panic!("ERROR: {:} al intentar obtener las piezas", err);
        }
    };

    get_play_result(pieces);
}
