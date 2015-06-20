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
}

impl Block {
    fn new(tetris: Tetromino) -> Block {
        Block {
            shape: tetris,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let x = Block::new(Tetromino::T);

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
