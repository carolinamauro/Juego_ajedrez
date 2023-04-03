#[cfg(test)]
mod test {
    use chess_game::{board::get_chess_pieces, piece::can_capture_piece};

    fn game_result(first_piece: char, second_piece: char) -> char {
        match first_piece {
            'P' => {
                if second_piece != 'P' {
                    return second_piece;
                } else {
                    return 'P';
                }
            }
            'W' | 'B' => {
                if second_piece == 'W' || second_piece == 'B' {
                    return 'E';
                } else {
                    return first_piece;
                }
            }
            _ => return 'E',
        }
    }
    #[test]
    fn test_tie() {
        let path = "tests/files/tie.txt".to_string();

        let pieces = match get_chess_pieces(path) {
            Ok(p) => p,
            Err(_) => return assert!(false),
        };

        let first_piece = can_capture_piece(&pieces.0, &pieces.1);
        let second_piece = can_capture_piece(&pieces.1, &pieces.0);
        assert_eq!('E', game_result(first_piece, second_piece));
    }

    #[test]
    fn test_white_victory() {
        let path = "tests/files/white_victory.txt".to_string();

        let pieces = match get_chess_pieces(path) {
            Ok(p) => p,
            Err(_) => return assert!(false),
        };

        let first_piece = can_capture_piece(&pieces.0, &pieces.1);
        let second_piece = can_capture_piece(&pieces.1, &pieces.0);
        assert_eq!('W', game_result(first_piece, second_piece));
    }

    #[test]
    fn test_black_victory() {
        let path = "tests/files/black_victory.txt".to_string();

        let pieces = match get_chess_pieces(path) {
            Ok(p) => p,
            Err(_) => return assert!(false),
        };

        let first_piece = can_capture_piece(&pieces.0, &pieces.1);
        let second_piece = can_capture_piece(&pieces.1, &pieces.0);
        assert_eq!('B', game_result(first_piece, second_piece));
    }

    #[test]
    fn test_losers() {
        let path = "tests/files/losers.txt".to_string();

        let pieces = match get_chess_pieces(path) {
            Ok(p) => p,
            Err(_) => return assert!(false),
        };

        let first_piece = can_capture_piece(&pieces.0, &pieces.1);
        let second_piece = can_capture_piece(&pieces.1, &pieces.0);
        assert_eq!('P', game_result(first_piece, second_piece));
    }

    #[test]
    fn test_file_no_exist() {
        let path = "".to_string();

        match get_chess_pieces(path) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, "No such file or directory (os error 2)"),
        };
    }

    #[test]
    fn test_panic() {
        let path = "tests/files/more_than_one_piece.txt".to_string();
        
        match get_chess_pieces(path) {
            Ok(_) =>  assert!(false),
            Err(e) => assert_eq!(e, "ERROR: se encontro más de 2 piezas"),
        };

    }
}
