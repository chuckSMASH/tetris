extern crate rand;

#[macro_use]
mod macros;
mod models;

use models::{ Direction, Grid, Tetriminos };


pub struct Game {
    grid: Grid,
    tetriminos: Tetriminos,
}


impl Game {
    pub fn run() {
        let mut game = Game {
            grid: Grid::new(20, 10),
            tetriminos: Tetriminos::init(),
        };
        let mut tet = game.tetriminos.next().unwrap();
        println!("Initial state:");
        println!("{:#?}", tet.blocks());
        let rotated = tet.rotate(&game.grid);
        println!("Rotated once ({}):", rotated);
        println!("{:#?}", tet.blocks());
        let shifted = tet.shift(Direction::Down, &game.grid);
        println!("Shifted down once ({}):", shifted);
        println!("{:#?}", tet.blocks());
        let rotated = tet.rotate(&game.grid);
        println!("Rotated once ({}):", rotated);
        println!("{:#?}", tet.blocks());
        let shift1 = tet.shift(Direction::Down, &game.grid);
        let shift2 = tet.shift(Direction::Down, &game.grid);
        println!("Shifted down twice ({}, {}):", shift1, shift2);
        println!("{:#?}", tet.blocks());
    }
}
