use crate::bitboard::Bitboard;
use crate::squares::{File, Square};

mod bitboard;
mod pieces;
mod board;
mod moves;
mod validation;
mod squares;

fn main() {
    let bitboard = Bitboard::from(Square {
        rank: 1,
        file: File::A
    });
    println!("{}", bitboard);
}
