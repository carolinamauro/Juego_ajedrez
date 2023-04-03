mod file_handler;

use crate::piece::{can_capture_piece, Pieces};
use std::io::Error;
use self::file_handler::read_file;

// Dado el path del archivo obtiene las piezas que se encuentran en el mismo
// En caso de que la pieza no exista paniquea. Si al leer el archivo ocurre un error
// se devuelve el mismo.
pub fn get_chess_pieces(file: &String) -> Result<(Pieces, Pieces), Error> {
    let pieces = match read_file(file) {
        Ok(p) => p,
        Err(er) => return Err(er),
    };
 
    let piece_0 = match pieces.0 {
        Some(p) => p,
        None => {
            panic!("ERROR: pieza leida no existe");
        }
    };

    let piece_1 = match pieces.1 {
        Some(p) => p,
        None => {
            panic!("ERROR: pieza leida no existe");
        }
    };

    return Ok((piece_0, piece_1));
}

// Imprime P W B o E según corresponda. 
// Toma la tupla pieces y obtiene para cada una el resultado de enfrentarse a la otra pieza.
pub fn get_play_result(pieces: (Pieces, Pieces)) {
    let first_piece = can_capture_piece(&pieces.0, &pieces.1);
    let second_piece = can_capture_piece(&pieces.1, &pieces.0);

    match first_piece {
        'P' => {
            if second_piece != 'P' {
                println!("{}", second_piece)
            } else {
                println!("{}", 'P')
            }
        }
        'W' | 'B' => {
            if second_piece == 'W' || second_piece == 'B' {
                println!("{}", 'E')
            } else {
                println!("{}", first_piece)
            }
        }
        _ => print!("{}", 'E'),
    }
}

#[test]
fn test_get_pieces() {
    let path = "./default.txt".to_string();

    match get_chess_pieces(&path) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }

    let path_wrong = "".to_string();

    match get_chess_pieces(&path_wrong) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}
