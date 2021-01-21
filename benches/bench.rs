#![feature(test)]

extern crate test;
use reversi::board::bitboard::{self, bitmask};

// #[bench]
// fn count_ones_bench(b: &mut test::Bencher) {
//     let s = test::black_box(0u64);
//     let e = test::black_box(10000u64);
//     b.iter(|| {
//         (s..e).fold(0, |a, b| a | bitboard::count_ones(b) )
//     });
// }
// #[bench]
// fn count_ones2_bench(b: &mut test::Bencher) {
//     let s = test::black_box(0u64);
//     let e = test::black_box(10000u64);
//     b.iter(|| {
//         (s..e).fold(0, |a, b| a | (b as i64).count_ones())
//     })
// }
#[bench]
fn first_set_bench(b: &mut test::Bencher) {
    use packed_simd::*;
    let s = test::black_box(bitmask::BLACK_INITIAL);
    let e = test::black_box(0x00000008100000ff);
    b.iter(|| {
        (s..e).fold(0, |a, _b| {
            a | bitboard::first_set(u64x4::splat(a)).extract(0)
        })
    })
}
#[bench]
fn rev_patt_bench(b: &mut test::Bencher) {
    let s = test::black_box(bitmask::BLACK_INITIAL);
    let e = test::black_box(0x00000008100000ff);
    let o = test::black_box(bitmask::WHITE_INITIAL);
    b.iter(|| (s..e).fold(0, |a, _b| a | bitboard::rev_patt(a, o, 26)))
}
// #[bench]
// fn rev_patt_simd__bench(b: &mut test::Bencher) {
//     let s = test::black_box(bitmask::BLACK_INITIAL);
//     let e = test::black_box(0x00000008100000ff);
//     let o = test::black_box(bitmask::WHITE_INITIAL);
//     b.iter(|| {
//         (s..e).fold(0, |a, b| a |
//             unsafe { bitboard::rev_patt_simd_(a, o, 26) }
//         )
//     })
// }
#[bench]
fn rev_patt_simd_bench(b: &mut test::Bencher) {
    let s = test::black_box(bitmask::BLACK_INITIAL);
    let e = test::black_box(0x00000008100000ff);
    let o = test::black_box(bitmask::WHITE_INITIAL);
    b.iter(|| (s..e).fold(0, |a, _b| a | bitboard::rev_patt_simd(a, o, 26)))
}
#[bench]
fn legal_patt_simd_bench(b: &mut test::Bencher) {
    let s = test::black_box(bitmask::BLACK_INITIAL);
    let e = test::black_box(bitmask::WHITE_INITIAL);
    b.iter(|| {
        bitboard::legal_patt_simd(s, e);
    })
}
