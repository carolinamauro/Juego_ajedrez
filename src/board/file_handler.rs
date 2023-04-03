use crate::piece::Pieces;
use crate::position::Position;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

fn read_file(file_name: &String) -> Result<(Option<Pieces>, Option<Pieces>), Error> {
    let mut current_pos: Position = Position::new(0, 0);
    let mut pieces: (Option<Pieces>, Option<Pieces>) = (None, None);

    match File::open(file_name) {
        Ok(file) => {
            let lines = BufReader::new(file).lines();
            for line in lines {
                current_pos.reset_y();
                if let Ok(current_line) = line {
                    for c in current_line.chars() {
                        match c {
                            '_' => current_pos.increase_y(),
                            ' ' => continue,
                            _ => match pieces.0.is_some() {
                                true => pieces.1 = Pieces::new(c, current_pos),
                                false => pieces.0 = Pieces::new(c, current_pos),
                            },
                        }
                    }
                    current_pos.increase_x();
                }
            }
            
            return Ok(pieces);
        }
        Err(e) => return Err(e),
    }
}

pub fn get_pieces(content_file: &String) -> Result<(Option<Pieces>, Option<Pieces>), Error> {
    let pieces = match read_file(content_file) {
        Ok(p) => p,
        Err(er) => return Err(er),
    };
    return Ok(pieces);
}

