mod file_handler;

use crate::piece::{can_capture_piece, Pieces};
use std::io::Error;

use file_handler::get_pieces;

pub fn get_chess_pieces(file: &String) -> Result<(Pieces, Pieces), Error> {
    let pieces = match get_pieces(file) {
        Ok(piece) => piece, //ver
        Err(err) => {
            return Err(err);
        }
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
