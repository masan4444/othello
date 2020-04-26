use std::io;
use std::io::{stdout, Write};
use othello::*;
// use othello::board::disp_bitboard;
use othello::board::*;

fn t(t: u64) -> u64 {
    t.rotate_right(8)
}

fn main() {
    // println!("Welcome to my othello world");
    // println!("Please choose play mode");
    // println!("Play with friend > input 0");
    // println!("Play with computer > input 1");
    // print!("mode > ");
    let a = U256::from_u64(1u64);
    let b = U256::from_u64(2u64);
    let c = &a >> 3usize;
    let d = &a - &b;
    let x = &a + &b;

    let mut board = Board::new();
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
        // let legal = board.legal_patt();
        let mut pos: usize = 0;
        println!{"You are {}", if board.turn { "⚫ BLACK ⚫" } else { "⚪ WHITE ⚪" }}
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
