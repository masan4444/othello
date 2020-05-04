use std::io;
use std::io::{stdout, Write};
use reversi::*;
use reversi::board::*;

fn main() {
    println!("Welcome to my reversi world");
    println!("Please choose play mode");
    println!("Play with friend > input 1");
    println!("Play with computer > input 2");
    print!("mode > ");
    io::stdout().flush().unwrap();
    let mut mode = String::new();
    io::stdin().read_line(&mut mode)
        .expect("Failed to read line");
    let mode: usize = mode.trim().parse()
        .expect("Please type a number!");
    let mut board = Board::new();
    if mode == 1 {
        loop {
            println!("{}", board);
            if board.is_finished() {
                println!("Finish!");
                break;
            }
            if board.is_pass() {
                println!("You passed!");
                board.turn = !board.turn;
                continue;
            }
            let mut pos: usize = 0;
            println!("You are {}", if board.turn { "⚫ BLACK ⚫" } else { "⚪ WHITE ⚪" });
            loop {
                print!("Please input coordinate (example: \"c4\" or \"C4\") > ");
                io::stdout().flush().unwrap();
                let mut coordinate = String::new();
                io::stdin().read_line(&mut coordinate)
                    .expect("Failed to read line");
                pos = match coordinate_to_pos(&coordinate.trim()) {
                    Some(pos) => {
                        if 1 << pos & board.legal_patt() == 0 {
                            println!("you can`t put there");
                            continue;
                        }
                        pos
                    },
                    None => {
                        println!("invalid input");
                        continue;
                    },
                };
                break;
            }
            println!("{}", pos);
            board.reverse(board.rev_patt(pos), pos);
            board.turn = !board.turn;
        }
    } else if mode == 2 {
        println!("now developing");
    } else {
        println!("???");
    }
}

fn coordinate_to_pos(cdn: &str) -> Option<usize> {
    if cdn.len() != 2 {
        return Option::None
    }
    let w = cdn.to_uppercase().chars()
                      .nth(0).unwrap() as isize
                      - 'A' as isize;
    let h = cdn.to_uppercase().chars()
                      .nth(1).unwrap() as isize
                      - '1' as isize;
    if h < 0 || h >= 8 || w < 0 || w >= 8 {
        return Option::None
    }
    Option::Some(63 - (w + h * 8) as usize)
}
fn pos_to_coordinate(pos: usize) -> (char, usize) {
    let w = ('H' as u8 - pos as u8 % 8) as char;
    (w, 8 - pos / 8)
}
