#[cfg(test)]
mod test {
    use chess_game::{
        board::get_chess_pieces,
        constants::{BLACK_WINS, LOSE_MOVE, TIE, WHITE_WINS},
        piece::can_capture_piece,
    };

    fn game_result(first_piece: char, second_piece: char) -> char {
        match first_piece {
            LOSE_MOVE => {
                if second_piece != LOSE_MOVE {
                    return second_piece;
                } else {
                    return LOSE_MOVE;
                }
            }
            WHITE_WINS | BLACK_WINS => {
                if second_piece == WHITE_WINS || second_piece == BLACK_WINS {
                    return TIE;
                } else {
                    return first_piece;
                }
            }
            _ => return TIE,
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
        assert_eq!(TIE, game_result(first_piece, second_piece));
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
        assert_eq!(WHITE_WINS, game_result(first_piece, second_piece));
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
        assert_eq!(BLACK_WINS, game_result(first_piece, second_piece));
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
        assert_eq!(LOSE_MOVE, game_result(first_piece, second_piece));
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
    fn test_more_pieces() {
        let path = "tests/files/more_than_one_piece.txt".to_string();

        match get_chess_pieces(path) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, "ERROR: se encontro más de 2 piezas"),
        };
    }

    #[test]
    fn test_unexisting_pieces() {
        let path = "tests/files/unexisting_pieces.txt".to_string();

        match get_chess_pieces(path) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, "ERROR: pieza leida no existe"),
        };
    }
}
