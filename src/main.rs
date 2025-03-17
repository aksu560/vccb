#![allow(dead_code)]

use crate::board::Board;

mod bitboard;
mod pieces;
mod board;
mod moves;
mod validation;
mod squares;

fn main() {
    println!("{}", Board::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
}
