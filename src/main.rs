enum Tetromino {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

struct Block {
    shape: Tetromino,
    anchor: (i32, i32),
}

impl Block {
    fn new(tetris: Tetromino) -> Block {
        Block {
            shape: tetris,
            anchor: (-1, -1),
        }
    }
}

struct Board {
    filled: bool,
}

impl Board {
    fn new(row: i32, col: i32) -> Vec<Vec<Board>> {

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

fn main() {
    // TODO: make it a dynamic board
    let R = 4;
    let C = 7;

    let mut board = Board::new(R, C);

    // TODO: put in a printing function
    for row in board {
        for col in row {
            match col.filled {
                true => print!("[X]"),
                false => print!("[ ]")
            }
        }
        println!("");
    }

    // Test data
    let mut blocks: Vec<Block> = vec![
        Block::new(Tetromino::I),
        Block::new(Tetromino::I),
        Block::new(Tetromino::T),
        Block::new(Tetromino::T),
        Block::new(Tetromino::S),
        Block::new(Tetromino::Z),
        Block::new(Tetromino::L)
    ];

    let x = &blocks[6];

    // test matching
    match x {
        &Block { shape: Tetromino::I, .. } => println!("I shape"),
        &Block { shape: Tetromino::O, .. } => println!("O shape"),
        &Block { shape: Tetromino::T, .. } => println!("T shape"),
        &Block { shape: Tetromino::J, .. } => println!("J shape"),
        &Block { shape: Tetromino::L, .. } => println!("L shape"),
        &Block { shape: Tetromino::S, .. } => println!("S shape"),
        &Block { shape: Tetromino::Z, .. } => println!("Z shape"),
    }
}

