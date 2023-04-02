use crate::piece::Piece;
use crate::position::Position;
use std::fs::File;
use std::io::ErrorKind;
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

fn get_pieces(content_file: String) -> Result<(Option<Piece>, Option<Piece>),ErrorKind> {
    let mut pieces: (Option<Piece>, Option<Piece>) = (None, None);
    let mut current_pos: Position = Position::new(0,0);

    if content_file.matches('_').count() == 62 && content_file.matches(' ').count() == 56 { //poner como contantesS
        for i in content_file.chars() {
            if i == '\n' {
                current_pos.increase_y();
                current_pos.reset_x()
            }
            else if i == '_' {
                current_pos.increase_x();
            }
            else if i != ' ' { //AGREGAR CHEQUEO EXTRA PIEZAS
                if pieces.0.is_some() { pieces.1 = Piece::new(i, current_pos) }
                else { pieces.0 = Piece::new(i, current_pos) } 
            }
        }
    }

    if pieces.0.is_some() && pieces.1.is_some() { 
        return Ok(pieces);
    }
    else {
        return Err(ErrorKind::InvalidData);
    }

    
} 

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
