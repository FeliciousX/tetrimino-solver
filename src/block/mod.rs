use super::coordinate::Coordinate;

/// The `Tetromino` type shows the 7 different types of Tetris Blocks.
pub enum Tetromino {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

/// Block represents the tetris blocks that will be placed in the `Board`
pub struct Block {
    pub shape: Tetromino,
    pub anchor: Coordinate,
    pub coordinates: Vec<Coordinate>,
    pub used: bool,
}

impl Block {
    pub fn new(tetris: Tetromino) -> Block {
        Block {
            shape: tetris,
            anchor: Coordinate::new(-1, -1),
            coordinates: vec![],
            used: false,
        }
    }

    pub fn get_shape(&self) -> &Tetromino {
        &self.shape
    }

    pub fn get_anchor(&self) -> &Coordinate {
        &self.anchor
    }

    pub fn get_coordinates(&self) -> &Vec<Coordinate> {
        &self.coordinates
    }

    pub fn get_used(&self) -> &bool {
        &self.used
    }

    pub fn set_anchor(&mut self, anchor: Coordinate) {
        self.anchor = anchor;
        // TODO: set all coordinates accordingly
    }

    /// Toggle the `used` attribute and returns the value
    pub fn toggle_used(&mut self) -> bool {
        self.used = !self.used;

        self.used
    }
}

