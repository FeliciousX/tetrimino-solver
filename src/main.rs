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
    fn new(width: i32, height: i32) -> Vec<Vec<Board>> {

        let mut board: Vec<Vec<Board>> = Vec::new();
        for w in 0..width {

            let mut innerBoard: Vec<Board> = Vec::new();
            for h in 0..height {
                innerBoard.push(Board { filled: false });
            }

            board.push(innerBoard);
        }

        return board;
    }
}

fn main() {
    println!("Hello, world!");
    let width = 7;
    let height = 4;

    let mut board = Board::new(width, height);
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

    match x.shape {
        Tetromino::I => println!("I shape"),
        Tetromino::O => println!("O shape"),
        Tetromino::T => println!("T shape"),
        Tetromino::J => println!("J shape"),
        Tetromino::L => println!("L shape"),
        Tetromino::S => println!("S shape"),
        Tetromino::Z => println!("Z shape"),
    }
}

