use crate::position::Position;


pub fn can_capture_piece_horizontal(current_piece: Position, piece_to_capture: Position) -> bool {
    return current_piece.x == piece_to_capture.x
}