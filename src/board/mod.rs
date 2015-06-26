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
    pub fn fill(&mut self, block: &block::Block) {
        // TODO: add block to the coordinate using the anchor coordinate as guide
        println!("filling");
        let r = block.get_row();
        let c = block.get_col();

        self.squares[r][c].filled = true;
    }
}

