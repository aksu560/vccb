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

impl From<&str> for File {
    fn from(s: &str) -> Self {
        match s.chars().next().unwrap() {
            'A' => File::A,
            'B' => File::B,
            'C' => File::C,
            'D' => File::D,
            'E' => File::E,
            'F' => File::F,
            'G' => File::G,
            'H' => File::H,
            _ => panic!("INVALID RANK WHEN PARSING!"),
        }
    }
}

impl From<u8> for File {
    fn from(v: u8) -> Self {
        match v {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => panic!("INVALID FILE WHEN PARSING"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square {
    pub rank: u8,
    pub file: File,
}

impl From<&str> for Square {
    fn from(s: &str) -> Self {
        Square {
            rank: String::from(s.chars().last().unwrap()).parse::<u8>().unwrap(),
            file: File::from(s)
        }
    }
}

/// Index to square
impl From<u8> for Square {
    fn from(s: u8) -> Self {
        Square {
            rank: (s / 8) + 1,
            file: File::from(s % 8),
        }
    }
}

/// Square to index
impl From<Square> for u8 {
    fn from(s: Square) -> Self {
        8 * (s.rank - 1) + s.file as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn u8_to_square_test() {
        assert_eq!(Square::from(0), Square { rank: 1, file: File::A });
        assert_eq!(Square::from(7), Square { rank: 1, file: File::H });
        assert_eq!(Square::from(8), Square { rank: 2, file: File::A });
        assert_eq!(Square::from(63), Square { rank: 8, file: File::H });
    }

    #[test]
    fn u8_from_square_test() {
        assert_eq!(u8::from(Square { rank: 1, file: File::A }), 0);
        assert_eq!(u8::from(Square { rank: 1, file: File::H }), 7);
        assert_eq!(u8::from(Square { rank: 2, file: File::A }), 8);
        assert_eq!(u8::from(Square { rank: 8, file: File::H }), 63);
    }
}
