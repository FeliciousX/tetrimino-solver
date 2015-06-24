mod block;
mod board;

use block::Block;
use block::Tetromino;
use board::Board;

fn main() {
    // TODO: make it a dynamic board
    let R = 4;
    let C = 7;

    let mut board = Board::new(R, C);

    // TODO: put in a printing function
    for row in board.squares {
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

