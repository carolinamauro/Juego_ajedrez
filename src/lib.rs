use board::{get_chess_pieces, get_play_result};
pub mod board;
pub mod piece;
pub mod position;

// Obtiene las piezas se encuentran en el archivo file pasado por parametro e imprime pantalla
// el resultado de la partida (sea E, P, B o W). En caso de ocurrir un error al leer y devolver las Piezas
// presentes en el archivo paniquea
pub fn print_move_result(file: String) -> Result<char, String> {
    let pieces = match get_chess_pieces(file) {
        Ok(p) => p,
        Err(err) => {
            return Err(err.to_string());
        }
    };
    return Ok(get_play_result(pieces));
}
