use crate::board::{Board, Sides};
use crate::pieces::Pieces;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square {
    rank: u8,
    file: File,
}

impl From<&str> for Square {
    fn from(s: &str) -> Self {
        Square {
            rank: String::from(s.chars().last().unwrap()).parse::<u8>().unwrap(),
            file: match s.chars().next().unwrap() {
                'A' => File::A,
                'B' => File::B,
                'C' => File::C,
                'D' => File::D,
                'E' => File::E,
                'F' => File::F,
                'G' => File::G,
                'H' => File::H,
                _ => panic!("INVALID RANK WHEN PARSING SQUARE!"),
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
    side: Sides,
    piece: Pieces,
    from: Square,
    to: Square
}

fn pseudo_legal_pawn_moves(side: Sides, from: Square, board: Board) -> Vec<Move> {
    let out = Vec::with_capacity(4);


    out
}