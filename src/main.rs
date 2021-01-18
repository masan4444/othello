extern crate reversi;

use std::process;

fn main() {
    println!("Welcome to my reversi world");
    let mode = reversi::set_play_mode().unwrap_or_else(|e| {
        eprintln!("Application Error: {}", e);
        process::exit(1)
    });
    println!("--- {} mode ---", mode.to_string());

    if let Err(e) = reversi::run(mode) {
        eprintln!("Application Error: {}", e);
        process::exit(1)
    };
}
