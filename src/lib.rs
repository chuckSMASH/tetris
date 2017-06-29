extern crate rand;

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
        let mut tet = game.tetriminos.take(1).next().unwrap();
        println!("{:?}", tet);
        println!("{:?}", tet.as_blocks());
        tet.rotate();
        println!("{:?}", tet.as_blocks());
        tet.rotate();
        println!("{:?}", tet.as_blocks());
        tet.rotate();
        println!("{:?}", tet.as_blocks());
    }
}
