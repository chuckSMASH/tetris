extern crate clap;
extern crate tetris;


use clap::{Arg, App};

use tetris::Game;


fn main() {
    let matches = App::new("Tetris")
        .version("0.1.0")
        .author("Chuck Bassett <iamchuckb@gmail.com>")
        .about("A game")
        .arg(Arg::with_name("level")
             .short("l")
             .long("level")
             .takes_value(true)
             .help("Starting level (0-20)"))
        .get_matches();

    let level_str = matches.value_of("level");
    let level: u8 = match level_str {
        None => 0u8,
        Some(s) => {
            match s.parse::<u8>() {
                Ok(n) if (n <= 20) => n,
                _ => panic!("Invalid start level :((("),
            }
        },
    };
    Game::run(level);
}
