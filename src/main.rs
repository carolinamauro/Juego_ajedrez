use std::env;

use chess_game::get_move_result;

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
   get_move_result(&file_name)

}
