use crate::bitboard::Bitboard;
use crate::pieces::Pieces::{KING, QUEEN, ROOK, BISHOP, KNIGHT, PAWN};

#[derive(Copy, Clone, Debug)]
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
    /// Bitboard.bb[Sides:White as usize][Pieces:Pawn as usize]
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

/// Create this type from the first part of a FEN-string.
impl From<&str> for Board {
    fn from(s: &str) -> Board {
        let mut b = Board::new();
        let mut square = 63;

        // Some flippy magic bs, because I'm lazy.
        let split = s.split('/').rev().collect::<Vec<&str>>();
        for char in split.join("").chars().rev() {
            match char {
                '1'..='8' => square -= char.to_digit(10).unwrap() as usize - 1,
                'P' => b.bb[Sides::White as usize][PAWN as usize].raw |= 1 << square,
                'N' => b.bb[Sides::White as usize][KNIGHT as usize].raw |= 1 << square,
                'B' => b.bb[Sides::White as usize][BISHOP as usize].raw |= 1 << square,
                'R' => b.bb[Sides::White as usize][ROOK as usize].raw |= 1 << square,
                'Q' => b.bb[Sides::White as usize][QUEEN as usize].raw |= 1 << square,
                'K' => b.bb[Sides::White as usize][KING as usize].raw |= 1 << square,
                'p' => b.bb[Sides::Black as usize][PAWN as usize].raw |= 1 << square,
                'n' => b.bb[Sides::Black as usize][KNIGHT as usize].raw |= 1 << square,
                'b' => b.bb[Sides::Black as usize][BISHOP as usize].raw |= 1 << square,
                'r' => b.bb[Sides::Black as usize][ROOK as usize].raw |= 1 << square,
                'q' => b.bb[Sides::Black as usize][QUEEN as usize].raw |= 1 << square,
                'k' => b.bb[Sides::Black as usize][KING as usize].raw |= 1 << square,
                _ => {}
            }
            square = square.wrapping_sub(1);
        }
        b
    }
}

#[cfg(test)]
mod tests {
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
        b.bb[Sides::White as usize][KING as usize] |= Bitboard::from(16);
        b.bb[Sides::Black as usize][KING as usize] |= Bitboard::from(1152921504606846976);

        assert_eq!(b.get_side(Sides::Both), Bitboard::from(1152921504606846992));
    }

    #[test]
    fn fen_test() {
        let b = Board::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

        assert_eq!(b.bb[Sides::White as usize][PAWN as usize], Bitboard::from(65280));
        assert_eq!(b.bb[Sides::White as usize][KNIGHT as usize], Bitboard::from(66));
        assert_eq!(b.bb[Sides::White as usize][BISHOP as usize], Bitboard::from(36));
        assert_eq!(b.bb[Sides::White as usize][ROOK as usize], Bitboard::from(129));
        assert_eq!(b.bb[Sides::White as usize][QUEEN as usize], Bitboard::from(8));
        assert_eq!(b.bb[Sides::White as usize][KING as usize], Bitboard::from(16));

        assert_eq!(b.bb[Sides::Black as usize][PAWN as usize], Bitboard::from(71776119061217280));
        assert_eq!(b.bb[Sides::Black as usize][KNIGHT as usize], Bitboard::from(4755801206503243776));
        assert_eq!(b.bb[Sides::Black as usize][BISHOP as usize], Bitboard::from(2594073385365405696));
        assert_eq!(b.bb[Sides::Black as usize][ROOK as usize], Bitboard::from(9295429630892703744));
        assert_eq!(b.bb[Sides::Black as usize][QUEEN as usize], Bitboard::from(576460752303423488));
        assert_eq!(b.bb[Sides::Black as usize][KING as usize], Bitboard::from(1152921504606846976));


    }
}