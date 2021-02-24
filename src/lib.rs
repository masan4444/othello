#[macro_use]

pub mod board;
pub mod com;
pub mod error;

use board::{Board, Color, Coordinate};
use std::convert::{From, TryFrom};
use std::error::Error;
use std::io;
use std::io::Write;

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

pub struct Opt {
    pub is_pvp: bool,
}

pub fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    let mut board = Board::new();
    let com_color = Color::WHITE;
    let is_pvp = opt.is_pvp;

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
            let pos = if !is_pvp && board.turn() == com_color {
                let (p, o) = board.bitboards();
                com::choose_pos_concurrency(p, o, board.count())
            } else {
                println!("You are {:?}", board.turn());
                let legal_patt = board.legal_patt();
                loop {
                    print!("Enter coordinate (example: \"c4\") > ");
                    let coordinate = read_line()?;
                    match Coordinate::try_from(&coordinate[..]) {
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

fn read_line() -> io::Result<String> {
    io::stdout().flush()?;
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim().to_string())
}
