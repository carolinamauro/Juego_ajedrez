use std::env;

use chess_game::print_move_result;

// Leo y devuelvo el archivo ingresado por comando.
// En caso de no haber ingresado un archivo se utiliza uno por default: "default.txt"
fn read_file_name(default_file: Option<String>) -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        return String::from(&args[1]);
    }

    if let Some(file) = default_file {
        return file;
    };

    panic!("Error: Especificar archivo")
}

fn main() {
    let file_name: String = read_file_name(Some("./default.txt".to_string()));
    print_move_result(&file_name);
}
