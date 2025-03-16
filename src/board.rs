use std::ops::Not;
use crate::bitboard::Bitboard;
use crate::pieces::Pieces::{KING, QUEEN, ROOK, BISHOP, KNIGHT, PAWN};
use crate::squares::Square;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Sides {
    White = 0,
    Black = 1,
    Both = 2,
    Neither = 3,
}

impl Not for Sides {
    type Output = Sides;
    fn not(self) -> Self::Output {
        match self {
            Sides::White => Sides::Black,
            Sides::Black => Sides::White,
            Sides::Both => Sides::Neither,
            Sides::Neither => Sides::Both,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Struct for board state.
pub struct Board {
    /// Collection of Bitboards associated with the board. The structure means we can do things like
    /// getting all white pawns like so:
    /// ```
    /// Bitboard.bb[Sides:White as usize][Pieces:Pawn as usize]
    /// ```
    pub bb: [[Bitboard; 6]; 2],
    pub to_move: Sides,
    pub castling_rights: [[bool; 2]; 2],
    pub en_passant: Option<Square>,
    pub progress: u64,
    pub current_move: u64,
}

impl Board {
    pub fn new() -> Board {
        Board {
            bb: [[Bitboard::new(); 6]; 2],
            to_move: Sides::White,
            castling_rights: [[false; 2]; 2],
            en_passant: None,
            progress: 0,
            current_move: 0,
        }
    }
    /// Get all pieces on side of a board.
    pub fn get_side(self, side: Sides) -> Bitboard {
        match side {
            Sides::Both => {
                self.get_side(Sides::White) | self.get_side(Sides::Black)
            },
            Sides::Neither => {
              !self.get_side(Sides::Both)
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

/// Create this type from a fen string.
impl From<&str> for Board {
    fn from(s: &str) -> Board {
        let mut b = Board::new();
        let mut square = 63;
        let fen_split = s.split(" ").collect::<Vec<&str>>();
        let moves = fen_split[0];

        // Board state parsing
        let moves_split = moves.split('/').rev().collect::<Vec<&str>>();
        for char in moves_split.join("").chars().rev() {
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
                _ => {panic!("FEN UNRECOGNIZED BOARD NOTATION SYMBOL!")}
            }
            square = square.wrapping_sub(1);
        }

        // Set turn
        b.to_move = match fen_split[1] {
            "w" => Sides::White,
            "b" => Sides::Black,
            _ => {panic!("FEN INVALID ACTIVE COLOR!")}
        };

        // Castling rights
        if fen_split[2].contains("K") {
            b.castling_rights[Sides::White as usize][KING as usize] = true;
        }
        if fen_split[2].contains("Q") {
            b.castling_rights[Sides::White as usize][QUEEN as usize] = true;
        }
        if fen_split[2].contains("k") {
            b.castling_rights[Sides::Black as usize][KING as usize] = true;
        }
        if fen_split[2].contains("q") {
            b.castling_rights[Sides::Black as usize][QUEEN as usize] = true;
        }

        // En Passant target square

        b.en_passant = match fen_split[3] {
            "-" => None,
            _ => Some(Square::from(fen_split[3].to_uppercase().as_ref()))
        };

        b.progress = fen_split[4].parse::<u64>().unwrap();

        b.current_move = fen_split[5].parse::<u64>().unwrap();

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
        assert_eq!(b.get_side(Sides::Neither), Bitboard::from(17293822569102704623));
    }

    #[test]
    fn fen_test_starting_pos() {
        let b = Board::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

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

        assert_eq!(b.to_move, Sides::White);
        assert_eq!(b.castling_rights, [[true; 2]; 2]);
        assert_eq!(b.en_passant, None);
        assert_eq!(b.progress, 0);
        assert_eq!(b.current_move, 1)
    }

    #[test]
    fn fen_test_perft_pos_3() {
        let b = Board::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 b Kq - 3 37");
        let mut target_castle = [[false; 2]; 2];
        target_castle[Sides::White as usize][KING as usize] = true;
        target_castle[Sides::Black as usize][QUEEN as usize] = true;

        assert_eq!(b.bb[Sides::White as usize][PAWN as usize], Bitboard::from(8589955072));
        assert_eq!(b.bb[Sides::White as usize][ROOK as usize], Bitboard::from(33554432));
        assert_eq!(b.bb[Sides::White as usize][KING as usize], Bitboard::from(4294967296));

        assert_eq!(b.bb[Sides::Black as usize][PAWN as usize], Bitboard::from(1134696536735744));
        assert_eq!(b.bb[Sides::Black as usize][ROOK as usize], Bitboard::from(549755813888));
        assert_eq!(b.bb[Sides::Black as usize][KING as usize], Bitboard::from(2147483648));

        assert_eq!(b.to_move, Sides::Black);
        assert_eq!(b.castling_rights, target_castle);
        assert_eq!(b.en_passant, None);
        assert_eq!(b.progress, 3);
        assert_eq!(b.current_move, 37)
    }
}