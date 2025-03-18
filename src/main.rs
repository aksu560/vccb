use crate::game::Game;
use crate::search::search;

mod game;
mod search;

fn main() {
    let g = Game::new();

    println!("{}", search(g));
}
