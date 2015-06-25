mod square;

use self::square::Square;
use super::block;


pub struct Board {
    pub squares: Vec<Vec<Square>>,
    height: i32,
    width: i32,
    complete: bool
}

impl Board {
    pub fn new(height: i32, width: i32) -> Board {
        let mut squares: Vec<Vec<Square>> = Vec::new();
        for _ in 0..height {

            let mut inner_square: Vec<Square> = Vec::new();
            for _ in 0..width {
                inner_square.push(Square { filled: false });
            }

            squares.push(inner_square);
        }

        Board {
            squares: squares,
            height: height,
            width: width,
            complete: false,
        }
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn is_complete(&self) -> bool {
        self.complete
    }

    pub fn set_complete(&mut self, status: bool) {
        self.complete = status;
    }

    pub fn fill(&mut self, block: &block::Block) {
        // TODO: add block to the coordinate using the anchor coordinate as guide
        println!("filling");
        let r = block.get_row();
        let c = block.get_col();

        self.squares[r][c].filled = true;
    }
}

