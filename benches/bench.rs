#![feature(test)]

extern crate test;
use othello::*;

// #[bench]
// fn count_ones_bench(b: &mut test::Bencher) {
//     let s = test::black_box(0u64);
//     let e = test::black_box(10000u64);
//     b.iter(|| {
//         (s..e).fold(0, |a, b| a | board::count_ones(b) )
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
fn rev_patt_bench(b: &mut test::Bencher) {
    let s = test::black_box(board::BIT_PATTERN::BLACK_INITIAL);
    let e = test::black_box(0x00000008100000ff);
    let o = test::black_box(board::BIT_PATTERN::WHITE_INITIAL);
    b.iter(|| {
        (s..e).fold(0, |a, b| a |
            board::rev_patt(a, o, 26)
        )
    })
}
// #[bench]
// fn rev_patt_simd__bench(b: &mut test::Bencher) {
//     let s = test::black_box(board::BIT_PATTERN::BLACK_INITIAL);
//     let e = test::black_box(0x00000008100000ff);
//     let o = test::black_box(board::BIT_PATTERN::WHITE_INITIAL);
//     b.iter(|| {
//         (s..e).fold(0, |a, b| a |
//             unsafe { board::rev_patt_simd_(a, o, 26) }
//         )
//     })
// }
#[bench]
fn rev_patt_simd_bench(b: &mut test::Bencher) {
    let s = test::black_box(board::BIT_PATTERN::BLACK_INITIAL);
    let e = test::black_box(0x00000008100000ff);
    let o = test::black_box(board::BIT_PATTERN::WHITE_INITIAL);
    b.iter(|| {
        (s..e).fold(0, |a, b| a |
            unsafe { board::rev_patt_simd(a, o, 26) }
        )
    })
}
#[bench]
fn legal_patt_bench(b: &mut test::Bencher) {
    let s = test::black_box(board::BIT_PATTERN::BLACK_INITIAL);
    let e = test::black_box(board::BIT_PATTERN::WHITE_INITIAL);
    b.iter(|| {
        board::legal_patt(s, e);
    })
}
#[bench]
fn legal_patt_simd_bench(b: &mut test::Bencher) {
    let s = test::black_box(board::BIT_PATTERN::BLACK_INITIAL);
    let e = test::black_box(board::BIT_PATTERN::WHITE_INITIAL);
    b.iter(|| {
        board::legal_patt_simd(s, e);
    })
}
