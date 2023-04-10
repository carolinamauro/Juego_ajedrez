use crate::piece::Pieces;
use crate::position::Position;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// Verifica si ya se encontraron una o dos piezas y agrega una nueva pieza a la tupla si todavía se puede
// Recibe un mutable reference a una tupla que contiene dos opciones de Pieces, un carácter c que representa el tipo de pieza a agregar
// y una referencia a una estructura Position que indica la posición actual en el tablero.
// La función devuelve un Result que indica si se pudo agregar la pieza correctamente o si ya se encontraron más de dos piezas.
// Si se pudo agregar la pieza, devuelve Ok(0) y si hubo un error devuelve Err(-1).

fn check_pieces(
    pieces: &mut (Option<Pieces>, Option<Pieces>),
    c: char,
    current_pos: &Position,
) -> Result<u32, i32> {
    let pos = match pieces.0.is_some() {
        true => {
            if pieces.1.is_some() {
                -1
            } else {
                1
            }
        }
        false => 0,
    };

    match pos {
        1 => {
            pieces.1 = Pieces::new(c, Position::new(current_pos.x, current_pos.y));
            return Ok(0);
        }
        0 => {
            pieces.0 = Pieces::new(c, Position::new(current_pos.x, current_pos.y));
            return Ok(0);
        }
        _ => return Err(-1),
    }
}

// Lee el archivo pasado por parametro y devuelve las dos piezas que se encuentan en el mismo
// Si falla retorna el error producido al inentar leer la linea del archivo
// Si la apretura del archivo tiene éxito, se crea un objeto BufReader que lee lína por línea el
// archivo. Se utiliza match para verificar si el caracter leído de la línea es ' ', '_' u otro.
//      - Si es un guión bajo, la posición actual en el eje Y se incrementa.
//      - Si es un espacio en blanco, se omite y se pasa al siguiente carácter.
//      - Si es cualquier otro carácter, se inicializa la pieza que corresponda. En caso de ya haber sido ambas inicializadas
//        se devuelve Error.

pub fn read_file(file_name: String) -> Result<(Option<Pieces>, Option<Pieces>), String> {
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
                            _ => match check_pieces(&mut pieces, c, &current_pos) {
                                Ok(0) => continue,
                                _ => return Err("ERROR: se encontro más de 2 piezas".to_string()),
                            },
                        }
                    }
                    current_pos.increase_x();
                }
            }

            return Ok(pieces);
        }
        Err(e) => return Err(e.to_string()),
    }
}
