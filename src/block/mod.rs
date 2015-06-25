use super::coordinate;

pub enum Tetromino {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

pub struct Block {
    pub shape: Tetromino,
    pub anchor: coordinate::Coordinate,
    pub coordinates: Vec<coordinate::Coordinate>,
    pub used: bool,
}

impl Block {
    pub fn new(tetris: Tetromino) -> Block {
        Block {
            shape: tetris,
            anchor: coordinate::Coordinate::new(-1, -1),
            coordinates: vec![],
            used: false,
        }
    }
}

