use std::fmt::{Display, Formatter};
use std::ops::{BitAnd, BitOr, BitXor, BitAndAssign, BitOrAssign, BitXorAssign, Not};

#[derive(Clone, Copy, Debug, PartialEq)]
/// Singular Bitboard.
pub struct Bitboard {
    /// Raw bits. u64 for convention/ease of use.
    raw: u64
}

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard { raw: 0 }
    }

    /// Create a 2D array of characters for visually representing a bitboard.
    pub fn get_display_arr(&self) -> [[char; 8]; 8] {
        let mut out = [[' '; 8]; 8];
        let mut internal = self.raw;

        for rank in (0..8).rev() {
            for file in 0..8 {
                out[rank][file] = match internal & 1 {
                    1 => 'X',
                    _ => '.'
                };
                internal >>= 1;
            }
        }
        out
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;
    fn bitand(self, other: Bitboard) -> Bitboard {
        Bitboard { raw: self.raw & other.raw }
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, other: Bitboard) {
        self.raw |= other.raw;
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;
    fn bitor(self, other: Bitboard) -> Bitboard {
        Bitboard { raw: self.raw | other.raw }
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, other: Bitboard) {
        self.raw |= other.raw;
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;
    fn bitxor(self, other: Bitboard) -> Bitboard {
        Bitboard { raw: self.raw ^ other.raw }
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, other: Bitboard) {
        self.raw ^= other.raw;
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let arr = self.get_display_arr();
        let mut out = String::with_capacity(arr.len());

        for rank in arr {
            for file in rank {
                out.push(file);
                out.push(' ');
            }
            out.push('\n');
        }
        write!(f, "{}", out)
    }
}

impl From<u64> for Bitboard {
    fn from(raw: u64) -> Bitboard {
        Bitboard { raw }
    }
}

impl Not for Bitboard {
    type Output = Bitboard;
    fn not(self) -> Bitboard {
        Bitboard { raw: !self.raw }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let bitboard = Bitboard::new();
        assert_eq!(bitboard.raw, 0);
    }

    #[test]
    fn from_u64_test() {
        let bitboard = Bitboard::from(1);
        assert_eq!(bitboard.raw, 1);
    }

    #[test]
    fn display_test() {
        let bitboard = Bitboard::from(7340935899360034705);
        let correct = "X . X . . X X . \n. . . . . X X X \n. X X X X X . . \n. X X X . . X . \n. . . X X X X . \n. . X . . . X . \nX X X X X X X . \nX . . . X . . X \n";
        assert_eq!(format!("{}", bitboard), correct);
    }
}