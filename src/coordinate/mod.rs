pub struct Coordinate {
    row: i32,
    col: i32,
}

impl Coordinate {
    pub fn new(row: i32, col: i32) -> Coordinate {
        Coordinate {
            row: row,
            col: col,
        }
    }

    pub fn get_row(&self) -> i32 {
        self.row
    }

    pub fn get_col(&self) -> i32 {
        self.col
    }
}
