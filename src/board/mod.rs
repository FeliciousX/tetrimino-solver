pub struct Board {
    pub filled: bool,
}

impl Board {
    pub fn new(row: i32, col: i32) -> Vec<Vec<Board>> {

        let mut board: Vec<Vec<Board>> = Vec::new();
        for w in 0..row {

            let mut innerBoard: Vec<Board> = Vec::new();
            for h in 0..col {
                innerBoard.push(Board { filled: false });
            }

            board.push(innerBoard);
        }

        return board;
    }

}

