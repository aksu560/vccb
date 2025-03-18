use cozy_chess::Board;

pub struct Game {
    pub board: Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: board_from_startpos()
        }
    }
}

impl Default for Game {
    fn default() -> Game {
        Game::new()
    }
}

impl From<Game> for Board {
    fn from(game: Game) -> Board {
        game.board
    }
}

pub fn board_from_startpos() -> Board {
    board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
}

pub fn board_from_fen(fen: &str) -> Board {
    fen.parse().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_from_startpos() {
        let board = board_from_startpos();
        let target = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".parse().unwrap();
        assert_eq!(board, target);
    }

    #[test]
    fn test_board_from_fen() {
        let board = board_from_fen("r1k2b1r/pb1nq1p1/2p1p1Bp/1p1n4/P2P4/5NB1/1PP2PPP/R2QR1K1 w - - 5 15");
        let target = "r1k2b1r/pb1nq1p1/2p1p1Bp/1p1n4/P2P4/5NB1/1PP2PPP/R2QR1K1 w - - 5 15".parse().unwrap();
        assert_eq!(board, target);
    }

    #[test]
    fn test_game_default() {
        let game = Game::default();
        assert_eq!(game.board, board_from_startpos());
    }
}