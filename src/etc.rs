#[inline]
fn delta_swap_64(bitboard: u64, mask: u64, delta: usize) -> u64 {
    let x = (bitboard ^ (bitboard >> delta)) & mask;
    bitboard ^ x ^ (x << delta)
}
#[inline]
fn flip_diag_a1_h8(bitboard: u64) -> u64 {
    let mut bitboard = delta_swap_64(bitboard, 0x00000000F0F0F0F0u64, 28);
    bitboard = delta_swap_64(bitboard, 0x0000CCCC0000CCCCu64, 14);
    delta_swap_64(bitboard, 0x00AA00AA00AA00AAu64, 7)
}
#[inline]
fn flip_vertical(bitboard: u64) -> u64 {
    bitboard.swap_bytes()
}
#[inline]
fn rotate_90_clockwise(bitboard: u64) -> u64 {
    flip_diag_a1_h8(flip_vertical(bitboard))
}
#[inline]
fn rotate_90_anti_clockwise(bitboard: u64) -> u64 {
    flip_vertical(flip_diag_a1_h8(bitboard))
}
#[inline]
fn rotate_180(bitboard: u64) -> u64 {
    bitboard.reverse_bits()
}

fn check_projection(f: fn(u64) -> u64) {
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
fn rotate_pseudo_45_anti_clockwise(bitboard: u64) -> u64 {
    const MASK1: u64 = 0xaaaaaaaaaaaaaaaa; //0xaaaaaaaaaaaaaaaa
    const MASK2: u64 = 0xcccccccccccccccc; //0xcccccccccccccccc
    const MASK3: u64 = 0xf0f0f0f0f0f0f0f0;
    let mut bitboard = bitboard ^ (MASK1 & (bitboard ^ bitboard.rotate_right(8)));
    bitboard = bitboard ^ (MASK2 & (bitboard ^ bitboard.rotate_right(16)));
    return bitboard ^ (MASK3 & (bitboard ^ bitboard.rotate_right(32)));
}
#[inline]
fn rotate_pseudo_45_clockwise(bitboard: u64) -> u64 {
    const MASK1: u64 = 0x5555555555555555; //0x5555555555555555
    const MASK2: u64 = 0x3333333333333333; //0x3333333333333333
    const MASK3: u64 = 0x0f0f0f0f0f0f0f0f;
    let mut bitboard = bitboard ^ (MASK1 & (bitboard ^ bitboard.rotate_right(8)));
    bitboard = bitboard ^ (MASK2 & (bitboard ^ bitboard.rotate_right(16)));
    return bitboard ^ (MASK3 & (bitboard ^ bitboard.rotate_right(32)));
}
