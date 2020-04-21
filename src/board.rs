use std::fmt;

type BitBoard = u64;

#[inline]
pub fn bitcount(bitboard: BitBoard) -> i32 {
    unsafe {
        core::arch::x86_64::_popcnt64(bitboard as i64)
    }
}

pub fn disp_bitboard(bitboard: BitBoard) {
    for i in (0..8).rev() {
        for j in (0..8).rev() {
            print!("{} ", if bitboard >> (i * 8 + j) & 1 == 1 { "●" } else { "□" });
        }
        println!("");
    }
    println!("");
}

pub fn get_rev_patt(player_bitboard: BitBoard, oppent_bitboard: BitBoard, pos: BitBoard) -> BitBoard {
    if (player_bitboard | oppent_bitboard) & pos != 0 {
        return 0
    }
    0
}


bitflags! {
    pub struct Pattern: u64 {
        const BLACK_INITIAL = 0x0000000810000000;
        const WHITE_INITIAL = 0x0000001008000000;
    }
}
#[derive(Debug)]
pub struct Board {
    pub black: BitBoard,
    pub white: BitBoard,
}
impl Board {
    pub fn new() -> Self {
        Self {
            black: Pattern::BLACK_INITIAL.bits(),
            white: Pattern::WHITE_INITIAL.bits(),
        }
    }
    pub fn reverse(&mut self, rev: BitBoard, pos: BitBoard, turn: bool) {
        match turn {
            true => {
                self.black ^= pos | rev;
                self.white ^= rev;
            },
            false => {
                self.white ^= pos | rev;
                self.black ^= rev;
            }
        }
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let legal_board = 0u64;
        let mut out = String::from("  ");
        for i in 0..8 {
            out.push_str(&format!("{} ", i));
        }
        out.push_str("← W\n");
        for i in (0..8).rev() {
            out.push_str(&format!("{} ", 7 - i));
            for j in (0..8).rev() {
                let check_bits = 1 << i * 8 + j;
                out.push_str(
                    match (self.black & check_bits, self.white & check_bits) {
                        (0, 0) => {
                            match legal_board & check_bits {
                                0 => "□ ",
                                _ => "◯ ",
                            }
                        }
                        (_, 0) => "⚫",
                        (0, _) => "⚪",
                        (_, _) => "X ",
                    }
                );
            }
            out.push_str(&format!("{}\n", 7 - i));
        }
        out.push_str("↑ ");
        for i in 0..8 {
            out.push_str(&format!("{} ", i))
        }
        out.push_str("\nH\n");
        write!(f, "{}", out)
    }
}
