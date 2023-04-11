use crate::position::Position;
#[derive(Clone, Copy, Debug)]
pub enum Color {
    Black,
    White,
}

// Estructura que se utiliza para guardar información de cada pieza del ajedrez
#[derive(Debug)]
pub struct PieceData(Color, Position);

impl PieceData {
    pub fn new(color: Color, pos: Position) -> Self {
        PieceData(color, pos)
    }
    pub fn get_color(&self) -> Color {
        self.0
    }
    pub fn get_position(&self) -> Position {
        self.1
    }
}
