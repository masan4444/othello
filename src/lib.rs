#[macro_use]

pub mod board;
pub mod com;
pub mod error;

use std::error::Error;
use std::fmt;
use std::io;
use std::io::Write;

use error::ApplicationError;

#[cfg(test)]
mod tests {
    use super::board;

    #[test]
    fn it_works() {
        let black: u64 = board::BIT_PATTERN::BLACK_INITIAL;
        let white: u64 = board::BIT_PATTERN::WHITE_INITIAL;
        assert_eq!((black | white).count_ones(), 4);
    }
}

#[derive(PartialEq)]
pub enum PlayMode {
    Computer = 1,
    Frind = 2,
}

impl fmt::Display for PlayMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlayMode::Computer => write!(f, "computer"),
            PlayMode::Frind => write!(f, "friend"),
        }
    }
}

pub fn set_play_mode() -> Result<PlayMode, Box<dyn Error>> {
    println!("Please choose play mode");
    println!("Play with friend > input 1");
    println!("Play with computer > input 2");
    print!("mode > ");
    io::stdout().flush()?;

    let mut mode = String::new();
    io::stdin().read_line(&mut mode)?;
    let mode = match mode.trim().parse()? {
        1 => PlayMode::Frind,
        2 => PlayMode::Computer,
        _ => Err(ApplicationError::InvalidModeError)?,
    };
    Ok(mode)
}

pub fn run(mode: PlayMode) -> Result<(), Box<dyn Error>> {
    let mut board = board::Board::new();
    let mut index = 0;
    let mut com_color = board::BLACK;
    println!("{}", board);

    loop {
        if board.is_finished() {
            println!("Finish!");
            let (black_count, white_count) = board.result();
            println!("BLACK: {}, WHITE: {}", black_count, white_count);
            if black_count == white_count {
                println!("draw!");
            } else {
                println!(
                    "{} wins!",
                    if black_count > white_count {
                        "BLACK"
                    } else {
                        "WHITE"
                    }
                );
            }
            break;
        } else if board.is_pass() {
            println!(
                "{} passed!",
                if board.turn {
                    "⚫ BLACK ⚫"
                } else {
                    "⚪ WHITE ⚪"
                }
            );
        } else {
            if mode == PlayMode::Computer && index == 0 {
                print!("Select your color > ");
                io::stdout().flush().unwrap();
                com_color = board::WHITE;
                println!("");
            }
            let pos: usize;
            if mode == PlayMode::Computer && board.turn == com_color {
                pos = unsafe {
                    com::choose_pos(board.board(com_color), board.board(!com_color), index)
                };
            } else {
                println!(
                    "You are {}",
                    if board.turn {
                        "⚫ BLACK ⚫"
                    } else {
                        "⚪ WHITE ⚪"
                    }
                );
                loop {
                    print!("Enter coordinate (example: \"c4\") > ");
                    io::stdout().flush().unwrap();
                    let mut coordinate = String::new();
                    io::stdin().read_line(&mut coordinate)?;
                    pos = match coordinate_to_pos(&coordinate.trim()) {
                        Some(pos) => {
                            if 1 << pos & board.legal_patt() == 0 {
                                println!("you can't put there");
                                continue;
                            }
                            pos
                        }
                        None => {
                            println!("invalid input");
                            continue;
                        }
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
    Ok(())
}

pub fn coordinate_to_pos(cdn: &str) -> Option<usize> {
    if cdn.len() != 2 {
        return None;
    }
    let w = cdn.to_uppercase().chars().nth(0).unwrap() as isize - 'A' as isize;
    let h = cdn.to_uppercase().chars().nth(1).unwrap() as isize - '1' as isize;
    if h < 0 || h >= 8 || w < 0 || w >= 8 {
        return None;
    }
    Some(63 - (w + h * 8) as usize)
}

#[allow(dead_code)]
fn pos_to_coordinate(pos: usize) -> (char, usize) {
    let w = ('H' as u8 - pos as u8 % 8) as char;
    (w, 8 - pos / 8)
}
