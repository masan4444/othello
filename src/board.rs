use std::fmt;
use std::ops;
use core::arch::x86_64::*;

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

pub mod BIT_PATTERN {
    pub const BLACK_INITIAL: u64     = 0x0000000810000000;
    pub const WHITE_INITIAL: u64     = 0x0000001008000000;

    pub const LOWER_MASK: u64        = 0x0080808080808080;
    pub const RIGHT_MASK: u64        = 0x7f00000000000000;
    pub const LOWER_LEFT_MASK: u64   = 0x0102040810204000;
    pub const LOWER_RIGHT_MASK: u64  = 0x0040201008040201;

    pub const UPPER_MASK: u64        = 0x0101010101010100;
    pub const LEFT_MASK: u64         = 0x00000000000000FE;
    pub const UPPER_RIGHT_MASK: u64  = 0x0002040810204080;
    pub const UPPER_LEFT_MASK: u64   = 0x8040201008040200;

    pub const SIDE_MASK: u64         = 0x7e7e7e7e7e7e7e7e;
    pub const UPPER_LEFT_CORNER: u64 = 0x8000000000000000;
    pub const LOWER_END_LINE: u64    = 0x00000000000000ff;

}

#[derive(Debug)]
pub enum U256 {
    M256i(__m256i),
}
impl U256 {
    #[inline]
    pub fn __m256i(&self) -> __m256i {
        match &self {
            Self::M256i(_data) => *_data
        }
    }
    #[inline]
    pub fn from_u64(val: u64) -> Self {
        Self::M256i(unsafe { _mm256_set1_epi64x(val as i64) })
    }
    #[inline]
    pub fn from_u64_4(x: u64, y: u64, z: u64, w: u64) -> Self {
        Self::M256i(unsafe { _mm256_set_epi64x(x as i64, y as i64, z as i64, w as i64) })
    }
    #[inline]
    pub fn and_not(&self, _rhs: &Self) -> U256 {
        Self::M256i(unsafe { _mm256_andnot_si256(self.__m256i(), _rhs.__m256i()) })
    }
    #[inline]
    pub fn first_set(&self) -> U256 {
        let flip_vertical_shuffle_table_256: __m256i = unsafe { _mm256_set_epi8(
            24, 25, 26, 27, 28, 29, 30, 31,
            16, 17, 18, 19, 20, 21, 22, 23,
             8,  9, 10, 11, 12, 13, 14, 15,
             0,  1,  2,  3,  4,  5,  6,  7
        ) };
        let mut data = self | &(self >> 1);
        data = &data | &(&data >> 2);
        data = &data | &(&data >> 4);
        data = (&data >> 1).and_not(&data);
        data = Self::M256i(unsafe { _mm256_shuffle_epi8(data.__m256i(), flip_vertical_shuffle_table_256) });
        data = &data & &-&data;
        data = Self::M256i(unsafe { _mm256_shuffle_epi8(data.__m256i(), flip_vertical_shuffle_table_256) });
        data
    }
    #[inline]
    pub fn nonzero(&self) -> U256 {
        return &Self::M256i(unsafe { _mm256_cmpeq_epi64(self.__m256i(), unsafe { _mm256_setzero_si256() }) }) + &Self::from_u64(1)
    }
    #[inline]
    pub fn  hor(&self) -> __m128i {
        let lhs_xz_yw: __m128i = unsafe { _mm_or_si128(_mm256_castsi256_si128(self.__m256i()),
            _mm256_extractf128_si256(self.__m256i(), 1)) };
        unsafe { _mm_or_si128(lhs_xz_yw, _mm_alignr_epi8(lhs_xz_yw, lhs_xz_yw, 8)) }
      }
}
impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::from("  ");

        write!(f, "{}", out)
    }
}

impl ops::Shr<usize> for &U256 {
    type Output = U256;
    #[inline]
    fn shr(self, n: usize) -> U256 {
        U256::M256i(unsafe { _mm256_srli_epi64(self.__m256i(), n as i32) })
    }
}
impl ops::Shl<usize> for &U256 {
    type Output = U256;
    #[inline]
    fn shl(self, n: usize) -> U256 {
        U256::M256i(unsafe { _mm256_slli_epi64(self.__m256i(), n as i32) })
    }
}
impl ops::BitAnd<Self> for &U256 {
    type Output = U256;
    #[inline]
    fn bitand(self, _rhs: Self) -> U256 {
        U256::M256i(unsafe { _mm256_and_si256(self.__m256i(), _rhs.__m256i()) })
    }
}
impl ops::BitOr<Self> for &U256 {
    type Output = U256;
    #[inline]
    fn bitor(self, _rhs: Self) -> U256 {
        U256::M256i(unsafe { _mm256_or_si256(self.__m256i(), _rhs.__m256i()) })
    }
}
impl<'a, 'b> ops::Add<&'b U256> for &'a U256 {
    type Output = U256;
    #[inline]
    fn add(self, _rhs: &'b U256) -> U256 {
        U256::M256i(unsafe { _mm256_add_epi64(self.__m256i(), _rhs.__m256i()) })
    }
}
impl ops::Sub<Self> for &U256 {
    type Output = U256;
    #[inline]
    fn sub(self, _rhs: Self) -> U256 {
        U256::M256i(unsafe { _mm256_sub_epi64(self.__m256i(), _rhs.__m256i()) })
    }
}
impl ops::Neg for &U256 {
    type Output = U256;
    #[inline]
    fn neg(self) -> U256 {
        U256::M256i(unsafe { _mm256_sub_epi64(_mm256_setzero_si256(), self.__m256i()) })
    }
}
impl ops::Not for &U256 {
    type Output = U256;
    #[inline]
    fn not(self) -> U256 {
        U256::M256i(unsafe { _mm256_andnot_si256(self.__m256i(), _mm256_set1_epi8(0xff as u8 as i8)) })
    }
}

// #[cfg_attr(target_arch = "x86_64", target_feature(enable = "popcnt"))]
#[inline]
pub fn count_ones(bitboard: u64) -> u32 {
    (bitboard as i64).count_ones()
}

pub fn disp_bitboard(bitboard: u64) {
    for i in (0..8).rev() {
        for j in (0..8).rev() {
            print!("{} ", match bitboard >> (i * 8 + j) & 1 {
                1 => "●",
                _ => "□"
            });
        }
        println!("");
    }
    println!("");
}

pub fn disp_bitboardx4(bitboards: __m256i) {
    disp_bitboard(unsafe { _mm_extract_epi64(_mm256_extractf128_si256(bitboards, 0), 0) as u64 });
    disp_bitboard(unsafe { _mm_extract_epi64(_mm256_extractf128_si256(bitboards, 0), 1) as u64 });
    disp_bitboard(unsafe { _mm_extract_epi64(_mm256_extractf128_si256(bitboards, 1), 0) as u64 });
    disp_bitboard(unsafe { _mm_extract_epi64(_mm256_extractf128_si256(bitboards, 1), 1) as u64 });
}

const HIGH_ORDER_MASKS: [u64; 4] = [
    BIT_PATTERN::LOWER_MASK,
    BIT_PATTERN::RIGHT_MASK,
    BIT_PATTERN::LOWER_LEFT_MASK,
    BIT_PATTERN::LOWER_RIGHT_MASK,
];
const LOW_ORDER_MASKS: [u64; 4] = [
    BIT_PATTERN::UPPER_MASK,
    BIT_PATTERN::LEFT_MASK,
    BIT_PATTERN::UPPER_RIGHT_MASK,
    BIT_PATTERN::UPPER_LEFT_MASK,
];
// #[inline]
pub fn rev_patt(p: u64, o: u64, pos: usize) -> u64 {
    let mut reversed = 0u64;
    let o_side_masked = o & BIT_PATTERN::SIDE_MASK;
    for (i, &mask) in HIGH_ORDER_MASKS.iter().enumerate() {
        let o = if i == 0 { o } else { o_side_masked };
        let mask = mask >> 63 - pos;
        let outflank = (BIT_PATTERN::UPPER_LEFT_CORNER >> (!o & mask).leading_zeros()) & p;
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
pub unsafe fn rev_patt_simd_(p: u64, o: u64, pos: usize) -> u64 {
    let o_side_masked = o & BIT_PATTERN::SIDE_MASK;
    let p = U256::from_u64(p);
    let o = U256::from_u64_4(o, o_side_masked, o_side_masked, o_side_masked);
    let mask = &U256::from_u64_4(
        BIT_PATTERN::LOWER_MASK,
        BIT_PATTERN::RIGHT_MASK,
        BIT_PATTERN::LOWER_LEFT_MASK,
        BIT_PATTERN::LOWER_RIGHT_MASK
    ) >> (63 - pos);
    let outflank = &o.and_not(&mask).first_set() & &p;
    let reversed = &(&-&outflank << 1) & &mask;
    let mask = &U256::from_u64_4(
        BIT_PATTERN::UPPER_MASK,
        BIT_PATTERN::LEFT_MASK,
        BIT_PATTERN::UPPER_RIGHT_MASK,
        BIT_PATTERN::UPPER_LEFT_MASK,
    ) << pos;
    let outflank = &(&mask & &(&(&o | &!&mask) + &U256::from_u64(1))) & &p;
    let reversed = &reversed | &(&(&(&outflank - &outflank.nonzero()) & &mask));
    let hor = reversed.hor();
    _mm_cvtsi128_si64(hor) as u64
}

#[inline]
pub unsafe fn _mm256_firstset_epi64(bits: __m256i) -> __m256i {
    let flip_vertical_shuffle_table_256: __m256i = _mm256_set_epi8(
        24, 25, 26, 27, 28, 29, 30, 31,
        16, 17, 18, 19, 20, 21, 22, 23,
         8,  9, 10, 11, 12, 13, 14, 15,
         0,  1,  2,  3,  4,  5,  6,  7
    );

    let mut bits = _mm256_or_si256(bits, _mm256_srli_epi64(bits, 1));
    bits = _mm256_or_si256(bits, _mm256_srli_epi64(bits, 2));
    bits = _mm256_or_si256(bits, _mm256_srli_epi64(bits, 4));
    bits = _mm256_andnot_si256(_mm256_srli_epi64(bits, 1), bits);
    bits = _mm256_shuffle_epi8(bits, flip_vertical_shuffle_table_256);
    bits = _mm256_and_si256(bits, _mm256_sub_epi64(_mm256_setzero_si256(), bits));
    _mm256_shuffle_epi8(bits, flip_vertical_shuffle_table_256)
}
#[inline]
pub unsafe fn _mm256_noeqzero_epi64(bits: __m256i) -> __m256i {
    _mm256_add_epi64(_mm256_cmpeq_epi64(bits, _mm256_setzero_si256()), _mm256_set1_epi64x(1))
}
#[inline]
pub unsafe fn rev_patt_simd(p: u64, o: u64, pos: usize) -> u64 {
    let o_side_masked = o & BIT_PATTERN::SIDE_MASK;
    let p = _mm256_set1_epi64x(p as i64);
    let o = _mm256_set_epi64x(o as i64, o_side_masked as i64, o_side_masked as i64, o_side_masked as i64);
    // let mask = () >> (63 - pos);
    let mask = _mm256_srli_epi64(_mm256_set_epi64x(
        BIT_PATTERN::LOWER_MASK as i64,
        BIT_PATTERN::RIGHT_MASK as i64,
        BIT_PATTERN::LOWER_LEFT_MASK as i64,
        BIT_PATTERN::LOWER_RIGHT_MASK as i64,
    ), 63 - pos as i32);
    // let outflank = (BIT_PATTERN::UPPER_LEFT_CORNER >> (!o & mask).leading_zeros()) & p;
    let outflank = _mm256_and_si256(_mm256_firstset_epi64(_mm256_andnot_si256(o, mask)), p);
    // let reversed = (-outflank << 1) & mask;
    let mut reserved = _mm256_and_si256(_mm256_slli_epi64(_mm256_sub_epi64(_mm256_setzero_si256(), outflank), 1), mask);
    // let mask = () << pos;
    let mask = _mm256_slli_epi64(_mm256_set_epi64x(
        BIT_PATTERN::UPPER_MASK as i64,
        BIT_PATTERN::LEFT_MASK as i64,
        BIT_PATTERN::UPPER_RIGHT_MASK as i64,
        BIT_PATTERN::UPPER_LEFT_MASK as i64,
    ), pos as i32);
    // let outflank = ((!mask | o) + 1) & mask & &p;
    let outflank =
        _mm256_and_si256(
            _mm256_and_si256(
                // (!mask | o) + 1
                _mm256_add_epi64(
                    // !mask | o
                    _mm256_or_si256(
                        // !mask
                        _mm256_andnot_si256(
                            mask, _mm256_set1_epi8(0xffu8 as i8)
                        ), o
                    ), _mm256_set1_epi64x(1)
                ), mask
            ), p
        );
    // let reversed = reversed | (((outflank - outflank != 0) & mask));
    reserved = _mm256_or_si256(
        reserved, _mm256_and_si256(
            // outflank - outflank != 0
            _mm256_sub_epi64(
                outflank, _mm256_noeqzero_epi64(outflank)
            ) ,mask
        )
    );
    let reseved = _mm_or_si128(_mm256_castsi256_si128(reserved),_mm256_extractf128_si256(reserved, 1));
    let tmp = _mm_or_si128(reseved, _mm_alignr_epi8(reseved, reseved, 8));
    _mm_cvtsi128_si64(tmp) as u64
}

#[inline]
fn delta_swap_64(bitboard: u64, mask: u64, delta: usize) -> u64 {
    let x = (bitboard ^ (bitboard >> delta)) & mask;
    bitboard ^ x ^ (x << delta)
}
#[inline]
pub fn flip_diag_A1H8(bitboard: u64) -> u64 {
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
    flip_diag_A1H8(flip_vertical(bitboard))
}
#[inline]
pub fn rotate_90_anti_clockwise(bitboard: u64) -> u64 {
    flip_vertical(flip_diag_A1H8(bitboard))
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
    bitboard =         bitboard ^ (MASK2 & (bitboard ^ bitboard.rotate_right(16)));
    return             bitboard ^ (MASK3 & (bitboard ^ bitboard.rotate_right(32)));
}
#[inline]
pub fn rotate_pseudo_45_clockwise(bitboard: u64) -> u64 {
    const MASK1: u64 = 0x5555555555555555; //0x5555555555555555
    const MASK2: u64 = 0x3333333333333333; //0x3333333333333333
    const MASK3: u64 = 0x0f0f0f0f0f0f0f0f;
    let mut bitboard = bitboard ^ (MASK1 & (bitboard ^ bitboard.rotate_right(8)));
    bitboard =         bitboard ^ (MASK2 & (bitboard ^ bitboard.rotate_right(16)));
    return             bitboard ^ (MASK3 & (bitboard ^ bitboard.rotate_right(32)));
}


#[inline]
fn legal_left_line(p: u8, o: u8) -> u8 {
    let p1 = p << 1;
    !(p1 | o) & (p1.wrapping_add(o))
}
#[inline]
fn legal_left(p: u64, o: u64) -> u64 {
    let mut legal = 0u64;
    for i in (0..64).step_by(8) {
        legal |= (legal_left_line(
            (p >> i & BIT_PATTERN::LOWER_END_LINE) as u8,
            (o >> i & BIT_PATTERN::LOWER_END_LINE) as u8
        ) as u64) << i;
    }
    legal
}
#[inline]
fn legal_horizontal(p: u64, o: u64) -> u64 {
    legal_left(p, o) | rotate_180(legal_left(rotate_180(p), rotate_180(o)))
}
#[inline]
fn legal_vertical(p: u64, o: u64) -> u64 {
    rotate_90_clockwise(
        legal_horizontal(
            rotate_90_anti_clockwise(p), rotate_90_anti_clockwise(o)
        )
    )
}
#[inline]
fn legal_diag_A8H1(p: u64, o: u64) -> u64 {
    let p = rotate_pseudo_45_clockwise(p);
    let o = rotate_pseudo_45_clockwise(o);
    const MASK: u64 = 0x80C0E0F0F8FCFEFF;
    let legal: u64 = (MASK & legal_horizontal(MASK & p, MASK & o)) |
    (!MASK & legal_horizontal(!MASK & p, !MASK & o));
    rotate_pseudo_45_anti_clockwise(legal.rotate_right(8))
}
#[inline]
fn legal_diag_A1H8(p: u64, o: u64) -> u64 {
    let p = rotate_pseudo_45_anti_clockwise(p);
    let o = rotate_pseudo_45_anti_clockwise(o);
    const MASK: u64 = 0xFEFCF8F0E0C08000;
    let legal: u64 = (MASK & legal_horizontal(MASK & p, MASK & o)) |
    (!MASK & legal_horizontal(!MASK & p, !MASK & o));
    rotate_pseudo_45_clockwise(legal.rotate_right(8))
}

pub fn legal_patt(p: u64, o: u64) -> u64 {
    legal_horizontal(p, o) | legal_vertical(p, o) |
    legal_diag_A1H8(p, o) | legal_diag_A8H1(p, o)
}


#[inline]
pub fn is_pass(p: u64, o: u64) -> bool {
    legal_patt(p, o) == 0 && legal_patt(o, p) != 0
}
#[inline]
pub fn is_finished(p: u64, o: u64) -> bool {
    legal_patt(p, o) == 0 && legal_patt(o, p) == 0
}

#[derive(Debug)]
pub struct Board {
    pub black: u64,
    pub white: u64,
    pub turn: bool,
}
impl Board {
    pub fn new() -> Self {
        Self {
            black: BIT_PATTERN::BLACK_INITIAL,
            white: BIT_PATTERN::WHITE_INITIAL,
            turn: true,
        }
    }
    pub fn reverse(&mut self, rev: u64, pos: usize) {
        let pos = 1u64 << pos;
        if self.turn {
            self.black ^= pos | rev;
            self.white ^= rev;
        } else {
            self.white ^= pos | rev;
            self.black ^= rev;
        }
    }
    pub fn is_pass(&self) -> bool {
        if self.turn {
            is_pass(self.black, self.white)
        } else {
            is_pass(self.white, self.black)
        }
    }
    pub fn is_finished(&self) -> bool {
        if self.turn {
            is_finished(self.black, self.white)
        } else {
            is_finished(self.white, self.black)
        }
    }
    pub fn legal_patt(&self) -> u64 {
        if self.turn {
            legal_patt(self.black, self.white)
        } else {
            legal_patt(self.white, self.black)
        }
    }
    pub fn rev_patt(&self, pos: usize) -> u64 {
        if self.turn {
            let new = unsafe { rev_patt_simd(self.black, self.white, pos) };
            let old = rev_patt(self.black, self.white, pos);
            assert_eq!(new, rev_patt(self.black, self.white, pos));
            new
        } else {
            let new = unsafe { rev_patt_simd(self.white, self.black, pos) };
            let old = rev_patt(self.white, self.black, pos);
            assert_eq!(new, rev_patt(self.white, self.black, pos));
            new
        }
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
                    match (self.black & check_bit, self.white & check_bit, legal_board & check_bit) {
                        (0, 0, 0) => "□ ", // blank
                        (0, 0, _) => "◯ ", // puttable
                        (_, 0, _) => "⚫",
                        (0, _, _) => "⚪",
                        (_, _, _) => "X ",
                    }
                );
            }
            out.push_str("\n");
        }
        out.push_str("↑ \nH\n");
        write!(f, "{}", out)
    }
}
