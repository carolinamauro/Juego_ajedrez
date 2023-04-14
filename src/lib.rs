use board::{get_chess_pieces, get_play_result};
pub mod board;
pub mod constants;
pub mod pieces;
pub mod position;
// Obtiene las piezas se encuentran en el archivo file pasado por parametro e imprime pantalla
// el resultado de la partida (sea E, P, B o W). En caso de ocurrir un error al leer y devolver las Piezas
// presentes en el archivo paniquea
pub fn print_move_result(file: String) -> Result<char, String> {
    let pieces = match get_chess_pieces(file) {
        Ok(p) => p,
        Err(err) => {
            return Err(err);
        }
    };
    let color_first_piece = pieces.0.get_color();
    if pieces.1.get_color().eq(&color_first_piece) {
        return Err("ERROR: Piezas con el mismo color".to_string());
    }
    Ok(get_play_result(pieces))
}
