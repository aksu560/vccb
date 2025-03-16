use crate::board::{Board, Sides};
use crate::moves::Move;
use crate::squares::Square;

impl Board {
    /// Returns a vector of all legal moves for a pawn on given square.
    fn pseudo_legal_pawn_moves(self, side: Sides, from: Square) -> Vec<Move> {
        let out: Vec<Move> = Vec::with_capacity(4);
        let ours = self.get_side(side);
        let theirs = self.get_side(!side);
        out
    }
}