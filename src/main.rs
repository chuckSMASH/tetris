#[macro_use] extern crate tetris;

fn main() {
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
