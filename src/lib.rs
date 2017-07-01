extern crate rand;

#[macro_use]
mod macros;
mod models;

use models::{ Grid, Tetriminos };


pub struct Game {
    grid: Grid,
    tetriminos: Tetriminos,
}


impl Game {
    pub fn run() {
        let game = Game {
            grid: Grid::new(20, 10),
            tetriminos: Tetriminos::init(),
        };
        let mut tet = game.tetriminos.take(1).next().unwrap();
        println!("{:#?}", tet);
        println!("{:?}", tet.blocks());
        tet.rotate(&game.grid);
        println!("{:?}", tet.blocks());
        tet.rotate(&game.grid);
        println!("{:?}", tet.blocks());
        tet.rotate(&game.grid);
        println!("{:?}", tet.blocks());
        tet.peek();
    }
}
