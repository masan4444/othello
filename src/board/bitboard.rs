use packed_simd::*;

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
fn noeqzero(bits: u64x4) -> u64x4 {
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
