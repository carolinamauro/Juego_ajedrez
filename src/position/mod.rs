
#[derive(Clone, Copy, Debug)]
pub struct Position {
    x: u32,
    y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
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

    pub fn reset_x(&mut self) {
        self.x = 0;
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