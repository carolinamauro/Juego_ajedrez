
#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position {
            x,
            y,
        }
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
        return self.x == pos.x
    }
    
    pub fn same_vertical(self, pos: &Position) -> bool {
        return self.y == pos.y
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
        return pos.same_position(&Position::new(dx, dy_left)) || pos.same_position(&Position::new(dx, dy_right));
    }

    pub fn same_diagonal_immediately_below(self, pos: &Position) -> bool {
        let dy_left: i32 = self.y - 1; 
        let dy_right: i32 = self.y + 1; 
        let dx: i32 = self.x + 1;
        return Position::new(dx, dy_left).same_position(pos)|| Position::new(dx, dy_right).same_position(pos);
    }
    
}


#[test]
fn test_increse() {
    let mut pos: Position = Position{x: 0, y: 0};
    pos.increase_x();
    pos.increase_y();
    assert_eq!(1, pos.x);
    assert_eq!(1, pos.y);
}