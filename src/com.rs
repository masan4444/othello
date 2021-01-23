use super::board::{bitboard, Coordinate};

// enum EvaluateMode {
//   Count,
//   Point,
//   Compound,
// }

// pub fn search(p: u64, o: u64, pos: usize, index: usize) -> usize {

//   0
// }

#[inline]
pub fn choose_pos(p: u64, o: u64, _index: usize) -> usize {
    let mut best_pos = 0;
    let mut legal_patt = bitboard::legal_patt_simd(p, o);
    let mut alpha = isize::MIN + 1;

    while legal_patt != 0 {
        let pos = legal_patt.trailing_zeros() as usize;
        let rev = bitboard::rev_patt_simd(p, o, pos);
        let score = -nega_alpha(
            o ^ rev,
            p ^ (1u64 << pos | rev),
            11,
            1,
            isize::MIN + 1,
            -alpha,
        );
        println!("pos: {}({}), score: {}", Coordinate::from(pos), pos, score);
        if score > alpha {
            best_pos = pos;
            alpha = score;
        }
        legal_patt &= !(1u64 << pos);
    }
    best_pos
}

#[inline]
pub fn evaluate(p: u64, o: u64, _legal_patt: u64, mode: isize) -> isize {
    match mode {
        1 => p.count_ones() as isize - o.count_ones() as isize,
        _ => 0,
    }
}

pub fn nega_alpha(p: u64, o: u64, depth: usize, mode: isize, alpha: isize, beta: isize) -> isize {
    let mut legal_patt = bitboard::legal_patt_simd(p, o);
    match (depth, legal_patt) {
        (0, _) => return evaluate(p, o, legal_patt, mode), // evaluate
        (_, 0) => {
            if bitboard::legal_patt_simd(o, p) == 0 {
                return evaluate(p, o, legal_patt, mode); // finish
            } else {
                return -nega_alpha(o, p, depth - 1, mode, -beta, -alpha); // pass
            }
        }
        (_, _) => (),
    }
    let mut alpha = alpha;
    while legal_patt != 0 {
        let pos = legal_patt.trailing_zeros() as usize;
        let rev = bitboard::rev_patt_simd(p, o, pos);
        let pos = 1u64 << pos;
        let score = -nega_alpha(o ^ rev, p ^ (pos | rev), depth - 1, mode, -beta, -alpha);
        alpha = if score > alpha { score } else { alpha };
        if alpha >= beta {
            return alpha;
        }
        legal_patt &= !pos;
    }
    alpha
}
