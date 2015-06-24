use super::block;

pub struct Board {
    pub squares: Vec<Vec<Square>>,
    pub row: i32,
    pub col: i32,
    pub complete: bool
}

pub struct Square {
   pub filled: bool,
}

impl Board {
    pub fn new(row: i32, col: i32) -> Board {
        let mut squares: Vec<Vec<Square>> = Vec::new();
        for w in 0..row {

            let mut innerSquare: Vec<Square> = Vec::new();
            for h in 0..col {
                innerSquare.push(Square { filled: false });
            }

            squares.push(innerSquare);
        }

        Board {
            squares: squares,
            row: row,
            col: col,
            complete: false,
        }
    }

    pub fn fill(block: block::Block, coordinate: (i32, i32)) {
        // TODO: add block to the coordinate using the anchor coordinate as guide
    }
}

