use super::board::*;
use packed_simd::*;

enum EvaluateMode {
  Count,
  Point,
  Compound,
}

// pub fn search(p: u64, o: u64, pos: usize, index: usize) -> usize {

//   0
// }

#[inline]
pub unsafe fn choose_pos(p: u64, o: u64, index: usize) -> usize {
  let mut best_pos = 0;
  let mut max_score = isize::MIN;
  let mut legal_patt = legal_patt_simd(p, o);
  while legal_patt != 0 {
    let pos = legal_patt.trailing_zeros() as usize;
    let rev = rev_patt_simd(p, o, pos);
    let score = match index {
      _ => -nega_alpha(o ^ rev, p ^ (1u64 << pos | rev), 11, 1),
    };
    let (coord_w, coord_h) = pos_to_coordinate(pos);
    println!("pos: {}{}({}), score: {}", coord_w, coord_h, pos, score);
    if score > max_score {
      best_pos = pos;
      max_score = score;
    }
    legal_patt &= !(1u64 << pos);
  }
  best_pos
}

#[inline]
pub fn evaluate(p: u64, o: u64, legal_patt: u64, mode: isize) -> isize {
  match mode {
    1 => p.count_ones() as isize - o.count_ones() as isize,
    _ => 0,
  }
}

pub unsafe fn _nega_alpha(
  p: u64,
  o: u64,
  depth: usize,
  mode: isize,
  alpha: isize,
  beta: isize,
) -> isize {
  let mut lagal_patt = legal_patt_simd(p, o);
  match (depth, lagal_patt) {
    (0, _) => return evaluate(p, o, lagal_patt, mode), // evaluate
    (_, 0) => {
      if legal_patt_simd(o, p) == 0 {
        return evaluate(p, o, lagal_patt, mode); // finish
      } else {
        return -_nega_alpha(o, p, depth - 1, mode, -beta, -alpha); // pass
      }
    }
    (_, _) => (),
  }
  let mut alpha = alpha;
  while lagal_patt != 0 {
    let pos = lagal_patt.trailing_zeros() as usize;
    let rev = rev_patt_simd(p, o, pos);
    let pos = 1u64 << pos;
    let score = -_nega_alpha(o ^ rev, p ^ (pos | rev), depth - 1, mode, -beta, -alpha);
    alpha = if score > alpha { score } else { alpha };
    if alpha >= beta {
      return alpha;
    }
    lagal_patt &= !pos;
  }
  alpha
}

#[inline]
pub unsafe fn nega_alpha(p: u64, o: u64, depth: usize, mode: isize) -> isize {
  return _nega_alpha(p, o, depth, mode, isize::MIN + 1, isize::MAX);
}
