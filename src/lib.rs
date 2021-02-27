#[macro_use]

pub mod board;
pub mod com;

use board::{Board, Color, Coordinate};
use std::convert::{From, TryFrom};
use std::error::Error;
use std::io;
use std::io::Write;

pub struct Opt {
    pub is_pvp: bool,
}

pub fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    let mut board = Board::default();
    let com_color = Color::WHITE;

    while !board.is_finished() {
        println!("{}", board);

        if board.is_pass() {
            println!("{:?} passed!", board.turn());
        } else {
            let pos = if !opt.is_pvp && board.turn() == com_color {
                // Com
                let (p, o) = board.bitboards();
                com::choose_pos_concurrency(p, o, board.count())
            } else {
                // Player
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
            println!();
            println!("{:?} chose: {}", board.turn(), Coordinate::from(pos));
            println!();
            board.reverse(board.rev_patt(pos), pos);
        }
        board.next();
    }

    println!("Finish!");
    println!("{}", board);
    let (black_count, white_count) = board.result();
    println!("BLACK: {}, WHITE: {}", black_count, white_count);
    if black_count == white_count {
        println!("draw!");
    } else {
        println!("{:?} wins!", Color::from(black_count > white_count));
    }
    Ok(())
}

fn read_line() -> io::Result<String> {
    io::stdout().flush()?;
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim().to_string())
}
