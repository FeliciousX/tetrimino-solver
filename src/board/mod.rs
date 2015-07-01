pub mod square {
    //! The Square struct represents a square on the board
    pub struct Square {
        /// `filled` is true if a block is sitting on top of it
        pub filled: bool,
    }
}

use self::square::Square;
use super::block;

/// Board represents the space the tetris has to add into
pub struct Board {
    /// 2D array of squares
    pub squares: Vec<Vec<Square>>,
    /// maximum row of board
    height: i32,
    /// maximum column of board
    width: i32,
    /// if the board is completely filled without gaps
    complete: bool
}

impl Board {
    /// Construct a new `Board` object.
    ///
    /// Accepts `height: i32` and `width: i32` as parameters to construct
    /// a board object with size `height x width`.
    ///
    /// # Examples
    /// ```
    /// use board::Board;
    ///
    /// let board = Board::new(5, 10);
    /// ```
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

    /// Gets `height` of board
    ///
    /// # Examples
    /// ```
    /// use board::Board;
    ///
    /// let board = Board::new(6, 12);
    /// height = board.get_height();
    ///
    /// assert_eq!(height, 6);
    /// ```
    pub fn get_height(&self) -> i32 {
        self.height
    }

    /// Gets `width` of board
    ///
    /// # Examples
    /// ```
    /// use board::Board;
    ///
    /// let board = Board::new(6, 12);
    /// width = board.get_width();
    ///
    /// assert_eq!(width, 12);
    /// ```
    pub fn get_width(&self) -> i32 {
        self.width
    }

    /// Check if `Board` has been completely filled perfectly.
    ///
    /// # Examples
    /// ```
    /// use board::Board;
    ///
    /// let board = Board::new(5, 10);
    ///
    /// assert_eq!(false, board.is_complete());
    /// ```
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Set `Board` completion status.
    ///
    /// # Examples
    /// ```
    /// use board::Board;
    ///
    /// let board = Board::new(4, 5);
    ///
    /// board.set_complete(true);
    ///
    /// assert_eq!(true, board.is_complete());
    /// ```
    pub fn set_complete(&mut self, status: bool) {
        self.complete = status;
    }

    /// Place a `Block` on the `Board`.
    ///
    /// The `Block` will be placed according to the `Block.anchor`
    ///
    /// # Examples
    /// ```
    /// use board::Board;
    /// use block::Block;
    ///
    /// let mut board = Board::new(10, 10);
    ///
    /// let mut block =  Block::new(Tetromino::I);
    /// let coordinate = Coordinate::new(5, 5);
    ///
    /// block.set_anchor(coordinate);
    /// board.fill(block);
    ///
    /// assert_eq!(true, board.squares[5][5].filled);
    /// ```
    pub fn fill(&mut self, block: &block::Block) {
        // TODO: add block to the coordinate using the anchor coordinate as guide
        println!("filling");
        let anchor = block.get_anchor();

        let r = anchor.get_row();
        let c = anchor.get_col();
        
        self.squares[r as usize][c as usize].filled = true;
    }
}

