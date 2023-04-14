mod file_handler;

use self::file_handler::read_file;
use crate::constants::{BLACK_WINS, LOSE_MOVE, TIE, WHITE_WINS};
use crate::pieces::Piece;

// Dado el path del archivo obtiene las piezas que se encuentran en el mismo
// En caso de que la pieza no exista o haya ocurrio un error, se devuelve el error.
pub fn get_chess_pieces(file: String) -> Result<(Piece, Piece), String> {
    let pieces = match read_file(file) {
        Ok(p) => p,
        Err(er) => return Err(er),
    };

    let piece_0 = match pieces.0 {
        Some(p) => p,
        None => return Err("ERROR: pieza leida no existe".to_string()),
    };

    let piece_1 = match pieces.1 {
        Some(p) => p,
        None => {
            return Err("ERROR: pieza leida no existe".to_string());
        }
    };

    Ok((piece_0, piece_1))
}

// Imprime P W B o E según corresponda.
// Toma la tupla pieces y obtiene para cada una el resultado de enfrentarse a la otra pieza.
pub fn get_play_result(pieces: (Piece, Piece)) -> char {
    let first_piece = Piece::capture_piece(&pieces.0, &pieces.1);
    let second_piece = Piece::capture_piece(&pieces.1, &pieces.0);

    match first_piece {
        LOSE_MOVE => {
            if second_piece != LOSE_MOVE {
                second_piece
            } else {
                LOSE_MOVE
            }
        }
        WHITE_WINS | BLACK_WINS => {
            if second_piece == WHITE_WINS || second_piece == BLACK_WINS {
                TIE
            } else {
                first_piece
            }
        }
        _ => TIE,
    }
}

#[test]
fn test_get_pieces_default() {
    let path = "./default.txt".to_string();

    match get_chess_pieces(path) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}
#[test]
fn test_get_pieces_wrong_file() {
    let path_wrong = "".to_string();

    match get_chess_pieces(path_wrong) {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e, "No such file or directory (os error 2)"),
    }
}
