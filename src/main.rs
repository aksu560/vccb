#![allow(dead_code)]

use crate::board::Board;
use crate::squares::{File, Square};

mod bitboard;
mod pieces;
mod board;
mod moves;
mod validation;
mod squares;

fn main() {
    let board = Board::new();

    board.rook_target_squares(Square {
        rank: 3,
        file: File::D
    });
}
