#[macro_use]
mod macros;


pub struct Game {}


impl Game {
    pub fn run() {
        let tetrimino_states = vec![
            states!("O"),
            states!("I"),
            states!("T"),
            states!("S"),
            states!("Z"),
            states!("J"),
            states!("L"),
        ];
        for tetrimino in &tetrimino_states {
            for state in tetrimino {
                println!("{}", state);
            }
            println!("{}", "=".repeat(10));
        }
    }
}
