use crate::piece::Piece;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn read_file(file_name: &String) -> Result<String,Error>  {

    let mut file: File = match File::open(file_name) {
        Ok(it) => it,
        Err(err) => {
            return Err(err)
        },
    }; 

    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Ok(it) => it,
        Err(err) => {
            return Err(err)
        },
    };

    return Ok(content);


}

pub fn get_chess_pieces(file: &String) {
    let mut pieces: (Piece, Piece);
    let mut file_readed = read_file(file);
    // return Some(pieces); -> Option<(Piece, Piece)>
}