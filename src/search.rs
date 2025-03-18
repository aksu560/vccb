use cozy_chess::{Move, Square};
use crate::game::Game;

pub fn search(game: Game) -> Move {
    let mut curr_mov = Move {
        from: Square::A1,
        to: Square::A1,
        promotion: None,
    };
    game.board.generate_moves(|moves| {
        let mut stop = false;
        for mv in moves {
            curr_mov = mv;
            stop = game.board.is_legal(mv);
            if stop {
                break;
            }
        }
        stop
    });
    curr_mov
}