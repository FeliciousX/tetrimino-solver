pub enum Tetromino {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

pub struct coordinate {
    pub x: i32,
    pub y: i32,
}

pub struct Block {
    pub shape: Tetromino,
    pub anchor: (i32, i32),
    pub coordinates: Vec<coordinate>,
    pub used: bool,
}

impl Block {
    pub fn new(tetris: Tetromino) -> Block {
        Block {
            shape: tetris,
            anchor: (-1, -1),
            coordinates: vec![],
            used: false,
        }
    }
}

