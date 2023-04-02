mod file_handle;

use crate::piece::Pieces;
use std::io::Error;


use file_handle::{get_pieces};

pub fn get_chess_pieces(file: &String) -> Result<(Option<Pieces>, Option<Pieces>), Error> {

    match get_pieces(file) {
        Ok(piece) => return Ok(piece), //ver
        Err(err) => { 
            return Err(err);
        }
    }

}
