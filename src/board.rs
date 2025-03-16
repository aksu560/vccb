use std::str::FromStr;
use crate::bitboard::Bitboard;

pub enum Sides {
    White = 0,
    Black = 1,
    Both = 2,
}

#[derive(Debug, Clone, Copy)]
/// Struct for board state.
pub struct Board {
    /// Collection of Bitboards associated with the board. The structure means we can do things like
    /// getting all white pawns like so:
    /// ```
    /// Bitboard.bb[Sides:White][Pieces:Pawn]
    /// ```
    pub bb: [[Bitboard; 6]; 2]
}

impl Board {
    pub fn new() -> Board {
        Board {
            bb: [[Bitboard::new(); 6]; 2]
        }
    }
    /// Get all pieces on side of a board.
    pub fn get_side(self, side: Sides) -> Bitboard {
        match side {
            Sides::Both => {
                self.get_side(Sides::White) | self.get_side(Sides::Black)
            },
            _ => {
                let mut out = Bitboard::new();
                for board in self.bb[side as usize] {
                    out |= board;
                }
                out
            }
        }
    }
}

/// Generate a Board struct from the first part of a FEN string.
impl From<String> for Board {
    fn from(_: String) -> Board {
        let mut bb = [[Bitboard::new(); 6]; 2];

        Board {
            bb
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pieces::Pieces;
    use super::*;

    #[test]
    fn new_board_test() {
        let b = Board::new();

        for side in b.bb.iter() {
            for board in side.iter() {
                assert_eq!(*board, Bitboard::new());
            }
        }
    }

    #[test]
    fn get_side_test() {
        let mut b = Board::new();
        b.bb[Sides::White as usize][Pieces::KING as usize] |= Bitboard::from(16);
        b.bb[Sides::Black as usize][Pieces::KING as usize] |= Bitboard::from(1152921504606846976);

        assert_eq!(b.get_side(Sides::Both), Bitboard::from(1152921504606846992));
    }
}