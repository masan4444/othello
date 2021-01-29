#[macro_use]

pub mod board;
pub mod com;
pub mod error;

use board::{Board, Color, Coordinate};
use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt;
use std::io;
use std::io::Write;

use error::ApplicationError;

#[cfg(test)]
mod tests {
    use super::board::bitboard::bitmask;

    #[test]
    fn it_works() {
        let black: u64 = bitmask::BLACK_INITIAL;
        let white: u64 = bitmask::WHITE_INITIAL;
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
    let mut board = Board::new();
    let com_color = Color::white();
    println!("{}", board);

    loop {
        if board.is_finished() {
            println!("Finish!");
            let (black_count, white_count) = board.result();
            println!("BLACK: {}, WHITE: {}", black_count, white_count);
            if black_count == white_count {
                println!("draw!");
            } else {
                println!("{:?} wins!", Color::from(black_count > white_count));
            }
            break;
        } else if board.is_pass() {
            println!("{:?} passed!", board.turn());
        } else {
            let pos = if mode == PlayMode::Computer && board.turn() == com_color {
                com::choose_pos(
                    board.board(com_color),
                    board.board(!com_color),
                    board.get_count(),
                )
            } else {
                println!("You are {:?}", board.turn());
                let legal_patt = board.legal_patt();
                loop {
                    print!("Enter coordinate (example: \"c4\") > ");
                    io::stdout().flush().unwrap();
                    let mut coordinate = String::new();
                    io::stdin().read_line(&mut coordinate)?;
                    match Coordinate::try_from(coordinate.trim()) {
                        Ok(cdn) if 1 << cdn.get_pos() & legal_patt != 0 => break cdn.get_pos(),
                        Ok(_) => println!("you can't put there"),
                        Err(e) => println!("Error: {}", e),
                    };
                }
            };
            println!("");
            println!("{:?} chose: {}", board.turn(), Coordinate::from(pos));
            println!("");
            board.reverse(board.rev_patt(pos), pos);
        }
        board.next();
        println!("{}", board);
    }
    Ok(())
}

fn read() -> io::Result<String> {
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s)
}
