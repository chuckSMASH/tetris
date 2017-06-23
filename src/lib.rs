#[macro_use]
mod macros;
mod models;

use models::Tetriminos;


pub struct Game {
    tetriminos: Tetriminos,
}


impl Game {
    pub fn run() {
        let game = Game {
            tetriminos: Tetriminos::init(),
        };
        for (tet_type, states) in game.tetriminos.states() {
            for state in states {
                println!("{:?}", tet_type);
                println!("{}", state);
            }
            println!("{}", "=".repeat(10));
        }
    }
}
