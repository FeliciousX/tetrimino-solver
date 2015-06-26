pub mod block;
pub mod board;
pub mod coordinate;

use block::Block;
use block::Tetromino;
use board::Board;

fn main() {
    // TODO: make it a dynamic board
    let r = 4;
    let c = 7;

    let mut board = Board::new(r, c);

    let coordinate = coordinate::Coordinate::new(3, 4);

    // Test data
    let blocks: Vec<Block> = vec![
        Block::new(Tetromino::I),
        Block::new(Tetromino::I),
        Block::new(Tetromino::T),
        Block::new(Tetromino::T),
        Block::new(Tetromino::S),
        Block::new(Tetromino::Z),
        Block::new(Tetromino::L),
        Block::new(Tetromino::O),
        Block::new(Tetromino::J)
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

}

