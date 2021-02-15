pub mod bitboard;

use bitboard::bitmask;
use std::fmt;

#[derive(Debug)]
pub struct Board {
    black: u64,
    white: u64,
    turn: Color,
    count: usize,
}
impl Board {
    pub fn new() -> Self {
        Self {
            black: bitmask::BLACK_INITIAL,
            white: bitmask::WHITE_INITIAL,
            turn: Color::BLACK,
            count: 0,
        }
    }
    pub fn get_turn(&self) -> Color {
        self.turn
    }
    pub fn get_count(&self) -> usize {
        self.count
    }
    pub fn bitboards(&self) -> (u64, u64) {
        if self.turn == Color::BLACK {
            (self.black, self.white)
        } else {
            (self.white, self.black)
        }
    }
    pub fn reverse(&mut self, rev: u64, pos: usize) {
        let pos = 1u64 << pos;
        if self.turn == Color::BLACK {
            self.black ^= pos | rev;
            self.white ^= rev;
        } else {
            self.white ^= pos | rev;
            self.black ^= rev;
        }
    }
    pub fn is_pass(&self) -> bool {
        let (p, o) = self.bitboards();
        bitboard::is_pass(p, o)
    }
    pub fn is_finished(&self) -> bool {
        let (p, o) = self.bitboards();
        bitboard::is_finished(p, o)
    }
    pub fn legal_patt(&self) -> u64 {
        let (p, o) = self.bitboards();
        bitboard::legal_patt_simd(p, o)
    }
    pub fn rev_patt(&self, pos: usize) -> u64 {
        let (p, o) = self.bitboards();
        bitboard::rev_patt_simd(p, o, pos)
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

extern crate derive_more;
use derive_more::{From, Not};

#[derive(Copy, Clone, PartialEq, From, Not)]
pub struct Color(bool);

impl Color {
    pub const BLACK: Color = Self(true);
    pub const WHITE: Color = Self(false);
}
impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", if self == &Color::BLACK { "BLACK" } else { "WHITE" })
    }
}
