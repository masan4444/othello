use packed_simd::*;
use std::fmt;

/*
------- bitboard pos ------
    A  B  C  D  E  F  G  H

1  MSB 62 61 60 59 58 57 56
2   55 54 53 52 51 50 49 48
3   47 46 45 44 43 42 41 40
4   39 38 37 36 35 34 33 32
5   31 30 29 28 27 26 25 24
6   23 22 21 20 19 18 17 16
7   15 14 13 12 11 10  9  8
8    7  6  5  4  3  2  1  0
                       (LSB)

---------------------------
*/

pub mod bitmask {
    pub const BLACK_INITIAL: u64 = 0x0000000810000000;
    pub const WHITE_INITIAL: u64 = 0x0000001008000000;

    pub const LOWER_MASK: u64 = 0x0080808080808080;
    pub const RIGHT_MASK: u64 = 0x7f00000000000000;
    pub const LOWER_LEFT_MASK: u64 = 0x0102040810204000;
    pub const LOWER_RIGHT_MASK: u64 = 0x0040201008040201;

    pub const UPPER_MASK: u64 = 0x0101010101010100;
    pub const LEFT_MASK: u64 = 0x00000000000000FE;
    pub const UPPER_RIGHT_MASK: u64 = 0x0002040810204080;
    pub const UPPER_LEFT_MASK: u64 = 0x8040201008040200;

    pub const SIDE_MASK: u64 = 0x7e7e7e7e7e7e7e7e;
    pub const UPPER_LEFT_CORNER: u64 = 0x8000000000000000;
    pub const LOWER_END_LINE: u64 = 0x00000000000000ff;

    pub const ALL_MASK: u64 = 0xffffffffffffffff;
}

pub fn disp_bitboard(bitboard: u64) {
    for i in (0..8).rev() {
        for j in (0..8).rev() {
            print!(
                "{} ",
                match bitboard >> (i * 8 + j) & 1 {
                    1 => "●",
                    _ => "□",
                }
            );
        }
        println!("");
    }
    println!("");
}

pub fn disp_bitboardx4(bitboards: u64x4) {
    for i in 0..4 {
        disp_bitboard(bitboards.extract(i));
    }
}

const HIGH_ORDER_MASKS: [u64; 4] = [
    bitmask::LOWER_MASK,
    bitmask::RIGHT_MASK,
    bitmask::LOWER_LEFT_MASK,
    bitmask::LOWER_RIGHT_MASK,
];
const LOW_ORDER_MASKS: [u64; 4] = [
    bitmask::UPPER_MASK,
    bitmask::LEFT_MASK,
    bitmask::UPPER_RIGHT_MASK,
    bitmask::UPPER_LEFT_MASK,
];
// #[inline]
pub fn rev_patt(p: u64, o: u64, pos: usize) -> u64 {
    let mut reversed = 0u64;
    let o_side_masked = o & bitmask::SIDE_MASK;
    for (i, &mask) in HIGH_ORDER_MASKS.iter().enumerate() {
        let o = if i == 0 { o } else { o_side_masked };
        let mask = mask >> 63 - pos;
        let outflank = (bitmask::UPPER_LEFT_CORNER >> (!o & mask).leading_zeros()) & p;
        reversed |= ((-(outflank as i64) as u64) << 1) & mask;
    }
    for (i, &mask) in LOW_ORDER_MASKS.iter().enumerate() {
        let o = if i == 0 { o } else { o_side_masked };
        let mask = mask << pos;
        let outflank = mask & ((o | !mask).wrapping_add(1)) & p;
        reversed |= (outflank - (outflank != 0) as u64) & mask;
    }
    reversed
}
#[inline]
pub fn first_set(bits: u64x4) -> u64x4 {
    let mut bits = bits | (bits >> 1);
    bits = bits | (bits >> 2);
    bits = bits | (bits >> 4);
    bits = bits | (bits >> 8);
    bits = bits | (bits >> 16);
    bits = bits | (bits >> 32);
    let lowers: u64x4 = bits >> 1;
    bits & !lowers
}
#[inline]
pub fn noeqzero(bits: u64x4) -> u64x4 {
    let zero = u64x4::splat(0);
    let mask = bits.ne(zero);
    let one = u64x4::splat(1);
    one & u64x4::from_cast(mask)
}
#[inline]
pub fn rev_patt_simd(p: u64, o: u64, pos: usize) -> u64 {
    let p = u64x4::splat(p);
    let o = u64x4::splat(o)
        & u64x4::new(
            bitmask::ALL_MASK,
            bitmask::SIDE_MASK,
            bitmask::SIDE_MASK,
            bitmask::SIDE_MASK,
        );
    let mask = u64x4::new(
        bitmask::LOWER_MASK,
        bitmask::RIGHT_MASK,
        bitmask::LOWER_LEFT_MASK,
        bitmask::LOWER_RIGHT_MASK,
    ) >> (63 - pos) as u32;
    let outflank = first_set(!o & mask) & p;
    let mut reversed = u64x4::from_cast(-i64x4::from_cast(outflank) << 1) & mask;
    let mask = u64x4::new(
        bitmask::UPPER_MASK,
        bitmask::LEFT_MASK,
        bitmask::UPPER_RIGHT_MASK,
        bitmask::UPPER_LEFT_MASK,
    ) << pos as u32;
    let outflank = mask & ((o | !mask) + 1) & p;
    reversed |= (outflank - noeqzero(outflank)) & mask;
    reversed.or()
}

#[inline]
fn delta_swap_64(bitboard: u64, mask: u64, delta: usize) -> u64 {
    let x = (bitboard ^ (bitboard >> delta)) & mask;
    bitboard ^ x ^ (x << delta)
}
#[inline]
pub fn flip_diag_a1_h8(bitboard: u64) -> u64 {
    let mut bitboard = delta_swap_64(bitboard, 0x00000000F0F0F0F0u64, 28);
    bitboard = delta_swap_64(bitboard, 0x0000CCCC0000CCCCu64, 14);
    delta_swap_64(bitboard, 0x00AA00AA00AA00AAu64, 7)
}
#[inline]
pub fn flip_vertical(bitboard: u64) -> u64 {
    bitboard.swap_bytes()
}
#[inline]
pub fn rotate_90_clockwise(bitboard: u64) -> u64 {
    flip_diag_a1_h8(flip_vertical(bitboard))
}
#[inline]
pub fn rotate_90_anti_clockwise(bitboard: u64) -> u64 {
    flip_vertical(flip_diag_a1_h8(bitboard))
}
#[inline]
pub fn rotate_180(bitboard: u64) -> u64 {
    bitboard.reverse_bits()
}

pub fn check_projection(f: fn(u64) -> u64) {
    let bitboard = 1u64;
    let mut projection = [0; 64];
    for i in 0..64 {
        projection[f(bitboard << i).leading_zeros() as usize] = i;
    }
    for i in 0..8 {
        for j in 0..8 {
            print!("{0: >2} ", projection[i * 8 + j])
        }
        println!("");
    }
    println!("");
}

/*
 63 62 61 60 59 58 57 56
 55 54 53 52 51 50 49 48
 47 46 45 44 43 42 41 40
 39 38 37 36 35 34 33 32
 31 30 29 28 27 26 25 24
 23 22 21 20 19 18 17 16
 15 14 13 12 11 10  9  8
  7  6  5  4  3  2  1  0

rotate_pseudo_45_anti_clockwise
 55 46 37 28 19 10  1 56
 47 38 29 20 11  2 57 48
 39 30 21 12  3 58 49 40
 31 22 13  4 59 50 41 32
 23 14  5 60 51 42 33 24
 15  6 61 52 43 34 25 16
  7 62 53 44 35 26 17  8
 63 54 45 36 27 18  9  0

rotate_pseudo_45_clockwise
 63  6 13 20 27 34 41 48
 55 62  5 12 19 26 33 40
 47 54 61  4 11 18 25 32
 39 46 53 60  3 10 17 24
 31 38 45 52 59  2  9 16
 23 30 37 44 51 58  1  8
 15 22 29 36 43 50 57  0
  7 14 21 28 35 42 49 56
*/
#[inline]
pub fn rotate_pseudo_45_anti_clockwise(bitboard: u64) -> u64 {
    const MASK1: u64 = 0xaaaaaaaaaaaaaaaa; //0xaaaaaaaaaaaaaaaa
    const MASK2: u64 = 0xcccccccccccccccc; //0xcccccccccccccccc
    const MASK3: u64 = 0xf0f0f0f0f0f0f0f0;
    let mut bitboard = bitboard ^ (MASK1 & (bitboard ^ bitboard.rotate_right(8)));
    bitboard = bitboard ^ (MASK2 & (bitboard ^ bitboard.rotate_right(16)));
    return bitboard ^ (MASK3 & (bitboard ^ bitboard.rotate_right(32)));
}
#[inline]
pub fn rotate_pseudo_45_clockwise(bitboard: u64) -> u64 {
    const MASK1: u64 = 0x5555555555555555; //0x5555555555555555
    const MASK2: u64 = 0x3333333333333333; //0x3333333333333333
    const MASK3: u64 = 0x0f0f0f0f0f0f0f0f;
    let mut bitboard = bitboard ^ (MASK1 & (bitboard ^ bitboard.rotate_right(8)));
    bitboard = bitboard ^ (MASK2 & (bitboard ^ bitboard.rotate_right(16)));
    return bitboard ^ (MASK3 & (bitboard ^ bitboard.rotate_right(32)));
}

pub fn legal_patt_simd(p: u64, o: u64) -> u64 {
    let shift1 = u64x4::new(1, 7, 9, 8);
    let mask = u64x4::new(
        bitmask::SIDE_MASK,
        bitmask::SIDE_MASK,
        bitmask::SIDE_MASK,
        bitmask::ALL_MASK,
    );
    let v_player = u64x4::splat(p);
    let masked_op = u64x4::splat(o) & mask;
    let mut flip_l = masked_op & (v_player << shift1);
    let mut flip_r = masked_op & (v_player >> shift1);
    flip_l |= masked_op & (flip_l << shift1);
    flip_r |= masked_op & (flip_r >> shift1);
    let pre_l = masked_op & (masked_op << shift1);
    let pre_r = pre_l >> shift1;
    let shift2 = shift1 + shift1;
    flip_l |= pre_l & (flip_l << shift2);
    flip_r |= pre_r & (flip_r >> shift2);
    flip_l |= pre_l & (flip_l << shift2);
    flip_r |= pre_r & (flip_r >> shift2);
    let mut res = flip_l << shift1;
    res |= flip_r >> shift1;
    res &= u64x4::splat(!(p | o));
    return res.or();
}

#[inline]
pub fn is_pass(p: u64, o: u64) -> bool {
    legal_patt_simd(p, o) == 0 && legal_patt_simd(o, p) != 0
}
#[inline]
pub fn is_finished(p: u64, o: u64) -> bool {
    legal_patt_simd(p, o) == 0 && legal_patt_simd(o, p) == 0
}

pub const BLACK: bool = true;
pub const WHITE: bool = false;

#[derive(Debug)]
pub struct Board {
    black: u64,
    white: u64,
    turn: bool,
    count: usize,
}
impl Board {
    pub fn new() -> Self {
        Self {
            black: bitmask::BLACK_INITIAL,
            white: bitmask::WHITE_INITIAL,
            turn: BLACK,
            count: 0,
        }
    }
    pub fn turn(&self) -> bool {
        self.turn
    }
    pub fn get_count(&self) -> usize {
        self.count
    }
    pub fn board(&self, color: bool) -> u64 {
        if color == BLACK {
            self.black
        } else {
            self.white
        }
    }
    pub fn reverse(&mut self, rev: u64, pos: usize) {
        let pos = 1u64 << pos;
        if self.turn == BLACK {
            self.black ^= pos | rev;
            self.white ^= rev;
        } else {
            self.white ^= pos | rev;
            self.black ^= rev;
        }
    }
    pub fn is_pass(&self) -> bool {
        is_pass(self.board(self.turn), self.board(!self.turn))
    }
    pub fn is_finished(&self) -> bool {
        is_finished(self.board(self.turn), self.board(!self.turn))
    }
    pub fn legal_patt(&self) -> u64 {
        legal_patt_simd(self.board(self.turn), self.board(!self.turn))
    }
    pub fn rev_patt(&self, pos: usize) -> u64 {
        rev_patt_simd(self.board(self.turn), self.board(!self.turn), pos)
    }
    pub fn next(&mut self) -> usize {
        self.turn = !self.turn;
        self.count += 1;
        self.count
    }
    pub fn result(&self) -> (u32, u32) {
        (self.black.count_ones(), self.white.count_ones())
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let legal_board = self.legal_patt();
        let mut out = String::from("  ");
        for i in 'A' as u8..'I' as u8 {
            out.push_str(&format!("{} ", i as char));
        }
        out.push_str("← W\n");
        for i in (0..8).rev() {
            out.push_str(&format!("{} ", 8 - i));
            for j in (0..8).rev() {
                let check_bit = 1 << i * 8 + j;
                out.push_str(
                    match (
                        self.black & check_bit,
                        self.white & check_bit,
                        legal_board & check_bit,
                    ) {
                        (0, 0, 0) => "- ", // blank
                        (0, 0, _) => "x ", // puttable
                        (_, 0, _) => "🔵",
                        (0, _, _) => "⭕",
                        (_, _, _) => "_  ",
                    },
                );
            }
            out.push_str("\n");
        }
        out.push_str("↑ \nH\n");
        write!(f, "{}", out)
    }
}

use std::convert::{From, TryFrom};

pub struct Coordinate {
    w: char,
    h: char,
}
impl Coordinate {
    fn try_new(w: char, h: char) -> Result<Self, &'static str> {
        let (_w, _h) = Self::char_to_index(w, h);
        if _h < 0 || _h >= 8 || _w < 0 || _w >= 8 {
            return Err("out of range");
        };
        Ok(Coordinate { w, h })
    }
    pub fn get_pos(&self) -> usize {
        let (w, h) = Self::char_to_index(self.w, self.h);
        63 - (w + h * 8) as usize
    }
    pub fn char_to_index(w: char, h: char) -> (isize, isize) {
        (w as isize - 'A' as isize, h as isize - '1' as isize)
    }
}
impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.w, self.h)
    }
}
impl From<usize> for Coordinate {
    fn from(pos: usize) -> Self {
        let w = ('H' as u8 - pos as u8 % 8) as char;
        let h = ('8' as u8 - pos as u8 / 8) as char;
        Coordinate { w, h }
    }
}
impl TryFrom<&str> for Coordinate {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 2 {
            return Err("invalid input");
        }
        let s = s.to_uppercase();
        let mut chars = s.chars();
        Coordinate::try_new(chars.next().unwrap(), chars.next().unwrap())
    }
}
