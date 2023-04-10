#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
// El struct Position que representa una posición en un tablero con varios métodos
//
// -  El método new inicializa una nueva instancia de Position con coordenadas x e y.
// -  Los métodos increase_x, increase_y, decrease_x, y decrease_y actualizan las coordenadas x y y de una instancia de Position
//    ya sea incrementando/decrementando en 1 el valor de x o y
// -  El método reset_y establece el valor de la coordenada y de una instancia de Position en cero.
// -  Los métodos same_position, same_horizontal, same_vertical, y same_diagonal comparan la posición de una instancia
//    de Position con otra instancia de Position para verificar si están en la misma posición, en la misma fila, en la misma
//    columna o en la misma diagonal.
// -  Los métodos same_diagonal_immediately_above y same_diagonal_immediately_below verifican si otra instancia de Position está
//    inmediatamente arriba o abajo en la diagonal de la instancia de Position.

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn increase_x(&mut self) {
        self.x += 1;
    }

    pub fn increase_y(&mut self) {
        self.y += 1;
    }

    pub fn decrease_x(&mut self) {
        self.x -= 1;
    }

    pub fn decrease_y(&mut self) {
        self.y -= 1;
    }

    pub fn reset_y(&mut self) {
        self.y = 0;
    }

    pub fn same_position(self, pos: &Position) -> bool {
        if self.x == pos.x && self.y == pos.y {
            return true;
        }
        return false;
    }

    pub fn same_horizontal(self, pos: &Position) -> bool {
        return self.x == pos.x;
    }

    pub fn same_vertical(self, pos: &Position) -> bool {
        return self.y == pos.y;
    }

    pub fn same_diagonal(self, pos: &Position) -> bool {
        let dx = self.x - pos.x;
        let dy = self.y - pos.y;

        if dx.abs() != dy.abs() {
            return false;
        }

        return true;
    }

    pub fn same_diagonal_immediately_above(self, pos: &Position) -> bool {
        let dy_left: i32 = self.y - 1;
        let dy_right: i32 = self.y + 1;
        let dx: i32 = self.x - 1;
        return pos.same_position(&Position::new(dx, dy_left))
            || pos.same_position(&Position::new(dx, dy_right));
    }

    pub fn same_diagonal_immediately_below(self, pos: &Position) -> bool {
        let dy_left: i32 = self.y - 1;
        let dy_right: i32 = self.y + 1;
        let dx: i32 = self.x + 1;
        return Position::new(dx, dy_left).same_position(pos)
            || Position::new(dx, dy_right).same_position(pos);
    }
}

#[test]
fn test_increse_decrese() {
    let mut pos: Position = Position { x: 3, y: 3 };
    pos.increase_x();
    pos.increase_y();
    assert_eq!(4, pos.x);
    assert_eq!(4, pos.y);
    pos.decrease_x();
    pos.decrease_y();
    assert_eq!(3, pos.x);
    assert_eq!(3, pos.y);
    assert!(Position::new(3, 3).same_position(&pos))
}

#[test]
fn test_horizontal() {
    let pos: Position = Position { x: 3, y: 3 };
    let mut pos_2: Position = Position { x: 3, y: 0 };
    assert!(pos.same_horizontal(&pos_2));
    pos_2.increase_x();
    assert_eq!(false, pos.same_horizontal(&pos_2));
}

#[test]
fn test_vertical() {
    let pos: Position = Position { x: 3, y: 7 };
    let mut pos_2: Position = Position { x: 1, y: 7 };
    assert!(pos.same_vertical(&pos_2));
    pos_2.decrease_y();
    assert_eq!(false, pos.same_vertical(&pos_2));
}

#[test]
fn test_diagonals() {
    let mut pos: Position = Position { x: 2, y: 2 };
    let mut pos_2: Position = Position { x: 5, y: 5 };
    assert!(pos.same_diagonal(&pos_2));
    pos_2.decrease_y();
    pos_2.decrease_x();
    assert!(pos.same_diagonal(&pos_2));
    pos.increase_x();
    assert_eq!(false, pos.same_diagonal(&pos_2));
}

#[test]
fn test_immediately_diagonals() {
    let pos: Position = Position { x: 5, y: 3 };
    let pos_2: Position = Position { x: 6, y: 4 };
    assert!(pos.same_diagonal_immediately_below(&pos_2));
    assert!(pos_2.same_diagonal_immediately_above(&pos));
}
