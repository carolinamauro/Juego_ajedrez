use std::env;

use chess_game::print_move_result;

// Leo y devuelvo el archivo ingresado por comando.
// En caso de no haber ingresado un archivo se utiliza uno por default: "default.txt"
// Si el arhivo default no se especifica, se devuelve Error-
fn read_file_name(default_file: Option<String>) -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        return Ok(String::from(&args[1]));
    }

    match default_file {
        Some(default_file) => return Ok(default_file),
        None => return Err("ERROR: especificar un archivo".to_string()),
    }
}

fn main() {
    let file_name: String;
    match read_file_name(Some("./default.txt".to_string())) {
        Ok(f) => file_name = f,
        Err(err) => {
            print!("{}", err);
            return;
        }
    }
    match print_move_result(file_name) {
        Ok(res) => print!("{}\n", res),
        Err(e) => print!("{}\n", e),
    }
}
