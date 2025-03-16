use crate::bitboard::Bitboard;

pub enum Sides {
    White = 0,
    Black = 1,
    Both = 2,
}

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub bb: [[Bitboard; 6]; 2]
}

impl Board {
    pub fn new() -> Board {
        Board {
            bb: [[Bitboard::new(); 6]; 2]
        }
    }

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