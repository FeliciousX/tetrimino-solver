pub mod block;
pub mod board;
pub mod coordinate;

use block::Block;
use block::Tetromino;
use board::Board;
use std::io;

fn main() {

    let board = init_board();

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

fn init_board() -> Board {
    let mut row = String::new();
    let mut column = String::new();

    println!("Enter number of rows:");
    
    io::stdin().read_line(&mut row)
        .ok()
        .expect("Failed to read line!");

    let row: i32 = row.trim().parse()
        .ok()
        .expect("Please type a number!");

    println!("Enter number of columns:");

    io::stdin().read_line(&mut column)
        .ok()
        .expect("Failed to read line!");

    let column: i32 = column.trim().parse()
        .ok()
        .expect("Please type a number!");

    return Board::new(row, column);
}

