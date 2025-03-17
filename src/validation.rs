use crate::bitboard::Bitboard;
use crate::board::{Board, Sides};
use crate::squares::{File, Square};

impl Board {
    pub fn white_pawn_target_squares(self, start: Square) -> Vec<Square> {
        let mut bb_acc: Bitboard = Bitboard::new();

        let single_mov = Bitboard::from(Square::from(u8::from(start) + 8));
        if (single_mov & self.get_side_bb(Sides::Both)).raw == 0 {
            bb_acc |= single_mov;

            if start.rank == 2  {
                let double_mov = Bitboard::from(Square::from(u8::from(start) + 16));
                if (double_mov & self.get_side_bb(Sides::Both)).raw == 0 {
                    bb_acc |= double_mov;
                }
            }
        }

        let mut attack_bb: Bitboard = Bitboard::new();
        if start.file != File::A {
            attack_bb |= Bitboard::from(Square::from(u8::from(start) + 7));
        }
        if start.file != File::H {
            attack_bb |= Bitboard::from(Square::from(u8::from(start) + 9));
        }

        let mut hostile_squares = self.get_side_bb(Sides::Black);

        match self.en_passant {
            None => {}
            Some(en_passant) => {
                hostile_squares |= Bitboard::from(en_passant)
            }
        }

        attack_bb &= hostile_squares;
        bb_acc |= attack_bb;
        Vec::<Square>::from(bb_acc)
    }

    pub fn black_pawn_target_squares(self, start: Square) -> Vec<Square> {
        let mut bb_acc: Bitboard = Bitboard::new();

        let single_mov = Bitboard::from(Square::from(u8::from(start) - 8));
        if (single_mov & self.get_side_bb(Sides::Both)).raw == 0 {
            bb_acc |= single_mov;

            if start.rank == 7  {
                let double_mov = Bitboard::from(Square::from(u8::from(start) - 16));
                if (double_mov & self.get_side_bb(Sides::Both)).raw == 0 {
                    bb_acc |= double_mov;
                }
            }
        }

        let mut attack_bb: Bitboard = Bitboard::new();
        if start.file != File::A {
            attack_bb |= Bitboard::from(Square::from(u8::from(start) - 7));
        }
        if start.file != File::H {
            attack_bb |= Bitboard::from(Square::from(u8::from(start) - 9));
        }

        let mut hostile_squares = self.get_side_bb(Sides::White);

        match self.en_passant {
            None => {}
            Some(en_passant) => {
                hostile_squares |= Bitboard::from(en_passant)
            }
        }

        attack_bb &= hostile_squares;
        bb_acc |= attack_bb;
        println!("{}", bb_acc);
        Vec::<Square>::from(bb_acc)
    }

    pub fn knight_target_squares(self, start: Square) -> Vec<Square> {
        // https://www.chessprogramming.org/Knight_Pattern
        let mut bb_acc = Bitboard::new();

        if start.file != File::A {

            if start.rank < 7 {
                bb_acc |= Bitboard::from(Square::from(u8::from(start) + 15));
            }
            if start.rank > 2 {
                bb_acc |= Bitboard::from(Square::from(u8::from(start) - 17));
            }

            if start.file != File::B {
                if start.rank < 8 {
                    bb_acc |= Bitboard::from(Square::from(u8::from(start) + 6));
                }
                if start.rank > 1 {
                    bb_acc |= Bitboard::from(Square::from(u8::from(start) - 10));
                }
            }
        }
        if start.file != File::H {

            if start.rank < 7 {
                bb_acc |= Bitboard::from(Square::from(u8::from(start) + 17));
            }
            if start.rank > 2 {
                bb_acc |= Bitboard::from(Square::from(u8::from(start) - 15));
            }

            if start.file != File::G {
                if start.rank < 8 {
                    bb_acc |= Bitboard::from(Square::from(u8::from(start) + 10));
                }
                if start.rank > 1 {
                    bb_acc |= Bitboard::from(Square::from(u8::from(start) - 6));
                }
            }
        }

        bb_acc &= !self.get_side_bb(self.to_move);
        println!("{}", bb_acc);
        Vec::<Square>::from(bb_acc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_pawn_target_squares() {
        // Double Move
        let board = Board::from("8/4p3/8/8/8/8/4P3/8 w - - 0 1");
        assert_eq!(board.white_pawn_target_squares(Square::from(12)).len(), 2);

        // Double Move (Blocked)
        let board = Board::from("8/8/8/8/4p3/8/4P3/8 w - - 0 1");
        assert_eq!(board.white_pawn_target_squares(Square::from(12)).len(), 1);

        // Double Move w/ 1 attack
        let board = Board::from("8/8/8/8/8/5p2/4P3/8 w - - 0 1");
        assert_eq!(board.white_pawn_target_squares(Square::from(12)).len(), 3);

        // Double Move w/ 2 attacks
        let board = Board::from("8/8/8/8/8/3p1p2/4P3/8 w - - 0 1");
        assert_eq!(board.white_pawn_target_squares(Square::from(12)).len(), 4);

        // Single Move
        let board = Board::from("8/8/8/8/8/3pPp2/8/8 w - - 0 1");
        assert_eq!(board.white_pawn_target_squares(Square::from(20)).len(), 1);
        // Sacré Bleu!
        let board = Board::from("rnbqkbnr/ppppp1pp/8/4Pp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 1");
        assert_eq!(board.white_pawn_target_squares(Square::from(36)).len(), 2);
    }

    #[test]
    fn test_black_pawn_target_squares() {
        // Double Move
        let board = Board::from("8/2p5/8/8/2P5/8/8/8 b - - 0 1");
        assert_eq!(board.black_pawn_target_squares(Square::from(Square {
            rank: 7,
            file: File::C,
        })).len(), 2);

        // Double Move (Blocked)
        let board = Board::from("8/2p5/8/2P5/8/8/8/8 b - - 0 1");
        assert_eq!(board.black_pawn_target_squares(Square::from(Square {
            rank: 7,
            file: File::C,
        })).len(), 1);

        // Double Move w/ 1 attack
        let board = Board::from("8/2p5/3P4/8/8/8/8/8 b - - 0 1");
        assert_eq!(board.black_pawn_target_squares(Square::from(Square {
            rank: 7,
            file: File::C,
        })).len(), 3);

        // Double Move w/ 2 attacks
        let board = Board::from("8/2p5/1P1P4/8/8/8/8/8 b - - 0 1");
        assert_eq!(board.black_pawn_target_squares(Square::from(Square {
            rank: 7,
            file: File::C,
        })).len(), 4);

        // Single Move
        let board = Board::from("8/8/1PpP4/8/8/8/8/8 b - - 0 1");
        assert_eq!(board.black_pawn_target_squares(Square::from(Square {
            rank: 6,
            file: File::C,
        })).len(), 1);

        // Sacré Bleu!
        let board = Board::from("rnbqkbnr/ppp1pppp/8/4N3/3pP3/8/PPPP1PPP/RNBQKB1R b KQkq e3 0 3");
        assert_eq!(board.black_pawn_target_squares(Square::from(Square {
            rank: 4,
            file: File::D,
        })).len(), 2);
    }

    #[test]
    fn test_knight_target_squares() {
        let b = Board::from("8/8/8/8/2N5/8/8/8 w - - 0 1");
        assert_eq!(b.knight_target_squares(Square::from(Square {
            rank: 4,
            file: File::C,
        })).len(), 8);

        let b = Board::from("8/8/3R4/8/2N5/8/8/8 w - - 0 1");
        assert_eq!(b.knight_target_squares(Square::from(Square {
            rank: 4,
            file: File::C,
        })).len(), 7);

        let b = Board::from("8/8/3R4/8/8/8/8/N7 w - - 0 1");
        assert_eq!(b.knight_target_squares(Square::from(Square {
            rank: 1,
            file: File::A,
        })).len(), 2);

        let b = Board::from("8/8/3R4/8/8/8/8/7N w - - 0 1");
        assert_eq!(b.knight_target_squares(Square::from(Square {
            rank: 8,
            file: File::A,
        })).len(), 2);
    }
}