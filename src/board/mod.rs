mod file_handle;

use crate::piece::Piece;
use std::io::ErrorKind;
use std::io::Error;

use file_handle::{read_file, get_pieces};

pub fn get_chess_pieces(file: &String) -> Result<(Piece, Piece), ErrorKind> {
    let mut pieces: (Option<Piece>, Option<Piece>) = (None, None);
    let mut file_readed: String;
    match read_file(file) {
        Ok(f) =>  file_readed = f,
        Err(err) =>  {
            return  Err(Error::kind(&err));
         } 
    };

    match get_pieces(file_readed) {
        Ok(piece) => return Ok((piece.0.unwrap(), piece.1.unwrap())), //ver
        Err(err) => { 
            return Err(err);
        }
    }

}
