use std::io;
use std::io::{Write};
use reversi::com;
use reversi::board::*;

const BLACK: bool = true;
const WHITE: bool = false;

fn main() {
    println!("Welcome to my reversi world");
    println!("Please choose play mode");
    println!("Play with friend > input 1");
    println!("Play with computer > input 2");
    print!("mode > ");
    io::stdout().flush().unwrap();

    let mut mode = String::new();
    io::stdin().read_line(&mut mode).expect("Failed to read line");
    let mode: usize = mode.trim().parse().expect("Please type a number!");
    println!("--- {} mode ---", if mode == 1 { "friend" } else { "computer" });

    let mut board = Board::new();
    let mut index = 0;
    let mut com_color = BLACK;
    println!("{}", board);

    loop {
        if board.is_finished() {
            println!("Finish!");
            let (black_count, white_count) = board.result();
            println!("BLACK: {}, WHITE: {}", black_count, white_count);
            if black_count == white_count {
                println!("draw!");
            } else {
                println!("{} wins!", if black_count > white_count { "BLACK" } else { "WHITE" });
            }
            break;
        } else if board.is_pass() {
            println!("{} passed!", if board.turn { "⚫ BLACK ⚫" } else { "⚪ WHITE ⚪" });
        } else {
            if mode == 2 && index == 0 {
                print!("Select your color > ");
                io::stdout().flush().unwrap();
                com_color = WHITE;
                println!("");
            }
            let pos: usize;
            if mode == 2 && board.turn == com_color {
                pos = unsafe {
                    com::choose_pos(board.board(com_color), board.board(!com_color), index)
                };
            } else {
                println!("You are {}", if board.turn { "⚫ BLACK ⚫" } else { "⚪ WHITE ⚪" });
                loop {
                    print!("Enter coordinate (example: \"c4\") > ");
                    io::stdout().flush().unwrap();
                    let mut coordinate = String::new();
                    io::stdin().read_line(&mut coordinate)
                        .expect("Failed to read line");
                    pos = match coordinate_to_pos(&coordinate.trim()) {
                        Some(pos) => {
                            if 1 << pos & board.legal_patt() == 0 {
                                println!("you can't put there");
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
            }
            board.reverse(board.rev_patt(pos), pos);
        }
        board.turn = !board.turn;
        index += 1;
        println!("{}", board);
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
