use crate::board::Sides;
use crate::pieces::Pieces;
use crate::squares::Square;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
    pub side: Sides,
    pub piece: Pieces,
    pub from: Square,
    pub to: Square,
    pub capture: bool,
    pub promote: Option<Pieces>,
}
