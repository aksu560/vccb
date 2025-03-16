use crate::board::{Board, Sides};
use crate::pieces::Pieces;
use crate::squares::Square;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
    side: Sides,
    piece: Pieces,
    from: Square,
    to: Square,
    capture: bool,
}
