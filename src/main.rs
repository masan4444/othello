use othello::*;

fn main() {
    println!("Welcome to my othello world");
    println!("Please choose play mode");
    println!("Play with friend > input 0");
    println!("Play with computer > input 1");
    print!("mode > ");
    let bitboard: u64 = 13;
    println!("{}", board::bitcount(bitboard));
    // board::disp_bitboard(bitboard);
    let board: board::Board = board::Board::new();
    println!("{}", board);
}
