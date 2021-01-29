use super::board::{bitboard, Coordinate};
use std::sync::{Arc, Mutex};
use std::thread;

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
    // nega_alpha(p, o, 11, 1)
    nega_alpha_concurrency(p, o, 11, 1)
}

#[inline]
pub fn choose_pos_concurrency(p: u64, o: u64, _index: usize) -> usize {
    let mut legal_patt = bitboard::legal_patt_simd(p, o);
    let mut alpha = isize::MIN + 1;

    let mut eldest_work_finished = false;
    let pos_and_scores = Arc::new(Mutex::new(Vec::<(usize, isize)>::new()));
    let mut handles = vec![];

    while legal_patt != 0 {
        let pos = legal_patt.trailing_zeros() as usize;
        let pos_and_scores_rc = Arc::clone(&pos_and_scores);

        let handle = thread::spawn(move || {
            println!("thread created");
            let rev = bitboard::rev_patt_simd(p, o, pos);
            let score = -_nega_alpha(
                o ^ rev,
                p ^ (1u64 << pos | rev),
                11,
                1,
                isize::MIN + 1,
                -alpha,
            );
            pos_and_scores_rc.lock().unwrap().push((pos, score));
            println!("pos: {}({}), score: {}", Coordinate::from(pos), pos, score);
        });

        if !eldest_work_finished {
            handle.join().unwrap();
            alpha = pos_and_scores.lock().unwrap().iter().next().unwrap().1;
            eldest_work_finished = true;
        } else {
            handles.push(handle);
        }
        legal_patt &= !(1u64 << pos);
        break;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut pos_and_scores = pos_and_scores.lock().unwrap();
    // for getting same result with non-concurrency
    pos_and_scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    pos_and_scores
        .iter()
        .max_by_key(|(_, score)| score)
        .unwrap()
        .0
}

#[inline]
pub fn nega_alpha_concurrency(p: u64, o: u64, depth: usize, mode: usize) -> usize {
    let mut legal_patt = bitboard::legal_patt_simd(p, o);

    let mut alpha = isize::MIN + 1;
    let beta = -alpha;

    let pos_and_scores = Arc::new(Mutex::new(Vec::<(usize, isize)>::new()));
    let thread_max: usize = 8;
    let thread_count: Arc<Mutex<isize>> = Arc::new(Mutex::new(0));
    let mut eldest_work_finished = false;
    let mut handles = vec![];

    while legal_patt != 0 {
        let pos = legal_patt.trailing_zeros() as usize;
        let rev = bitboard::rev_patt_simd(p, o, pos);
        let next_p = o ^ rev;
        let next_o = p ^ (1u64 << pos | rev);
        let pos_and_scores_rc = Arc::clone(&pos_and_scores);
        let thread_count_rc = Arc::clone(&thread_count);

        if !eldest_work_finished || *thread_count_rc.lock().unwrap() >= thread_max as isize {
            let score = -_nega_alpha_concurrency(
                next_p,
                next_o,
                depth,
                1,
                -beta,
                -alpha,
                thread_count_rc,
                thread_max,
            );
            println!("pos: {}({}), score: {}", Coordinate::from(pos), pos, score);
            pos_and_scores_rc.lock().unwrap().push((pos, score));
            alpha = std::cmp::max(alpha, score);
            eldest_work_finished = true;
        } else {
            *thread_count_rc.lock().unwrap() += 1;
            let thread_count_rc2 = Arc::clone(&thread_count);
            let handle = thread::spawn(move || {
                let score = -_nega_alpha_concurrency(
                    next_p,
                    next_o,
                    depth,
                    1,
                    -beta,
                    -alpha,
                    thread_count_rc,
                    thread_max,
                );
                println!("pos: {}({}), score: {}", Coordinate::from(pos), pos, score);
                pos_and_scores_rc.lock().unwrap().push((pos, score));
                *thread_count_rc2.lock().unwrap() -= 1;
            });
            handles.push(handle);
        }
        legal_patt &= !(1u64 << pos);
        break;
    }
    for handle in handles {
        handle.join().unwrap();
        *thread_count.lock().unwrap() -= 1;
    }
    let mut pos_and_scores_rc = pos_and_scores.lock().unwrap();
    // for getting same result with non-concurrency
    pos_and_scores_rc.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    pos_and_scores_rc
        .iter()
        .max_by_key(|(_, score)| score)
        .unwrap()
        .0
}

#[inline]
pub fn evaluate(p: u64, o: u64, _legal_patt: u64, mode: isize) -> isize {
    match mode {
        1 => p.count_ones() as isize - o.count_ones() as isize,
        _ => 0,
    }
}

fn _nega_alpha(p: u64, o: u64, depth: usize, mode: isize, alpha: isize, beta: isize) -> isize {
    let mut legal_patt = bitboard::legal_patt_simd(p, o);
    match (depth, legal_patt) {
        (0, _) => return evaluate(p, o, legal_patt, mode), // evaluate
        (_, 0) => {
            if bitboard::legal_patt_simd(o, p) == 0 {
                return evaluate(p, o, legal_patt, mode); // finish
            } else {
                return -_nega_alpha(o, p, depth - 1, mode, -beta, -alpha); // pass
            }
        }
        (_, _) => (),
    }
    let mut alpha = alpha;
    while legal_patt != 0 {
        let pos = legal_patt.trailing_zeros() as usize;
        let rev = bitboard::rev_patt_simd(p, o, pos);
        let pos = 1u64 << pos;
        let score = -_nega_alpha(o ^ rev, p ^ (pos | rev), depth - 1, mode, -beta, -alpha);
        alpha = if score > alpha { score } else { alpha };
        if alpha >= beta {
            return alpha;
        }
        legal_patt &= !pos;
    }
    alpha
}

pub fn nega_alpha(p: u64, o: u64, depth: usize, mode: isize) -> usize {
    let mut best_pos = 0;
    let mut legal_patt = bitboard::legal_patt_simd(p, o);
    let mut alpha = isize::MIN + 1;

    while legal_patt != 0 {
        let pos = legal_patt.trailing_zeros() as usize;
        let rev = bitboard::rev_patt_simd(p, o, pos);
        let score = -_nega_alpha(
            o ^ rev,
            p ^ (1u64 << pos | rev),
            depth,
            mode,
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

pub fn _nega_alpha_concurrency(
    p: u64,
    o: u64,
    depth: usize,
    mode: isize,
    alpha: isize,
    beta: isize,
    thread_count: Arc<Mutex<isize>>,
    thread_max: usize,
) -> isize {
    let mut legal_patt = bitboard::legal_patt_simd(p, o);
    match (depth, legal_patt) {
        (0, _) => return evaluate(p, o, legal_patt, mode), // evaluate
        (_, 0) => {
            if bitboard::legal_patt_simd(o, p) == 0 {
                return evaluate(p, o, legal_patt, mode); // finish
            } else {
                return -_nega_alpha_concurrency(
                    o,
                    p,
                    depth - 1,
                    mode,
                    -beta,
                    -alpha,
                    thread_count,
                    thread_max,
                ); // pass
            }
        }
        (_, _) => (),
    }
    let mut alpha = alpha;
    let mut eldest_work_finished = false;
    let scores = Arc::new(Mutex::new(Vec::<isize>::new()));
    let mut handles = vec![];

    while legal_patt != 0 {
        let pos = legal_patt.trailing_zeros() as usize;
        let rev = bitboard::rev_patt_simd(p, o, pos);
        let next_p = o ^ rev;
        let next_o = p ^ (1u64 << pos | rev);
        let scores_rc = Arc::clone(&scores);
        let thread_count_rc = Arc::clone(&thread_count);

        if !eldest_work_finished || *thread_count_rc.lock().unwrap() >= thread_max as isize {
            let score = -_nega_alpha_concurrency(
                next_p,
                next_o,
                depth - 1,
                1,
                -beta,
                -alpha,
                thread_count_rc,
                thread_max,
            );
            scores_rc.lock().unwrap().push(score);
            alpha = std::cmp::max(alpha, score);
            if alpha >= beta {
                return alpha;
            }
            // eldest_work_finished = true;
        } else {
            *thread_count_rc.lock().unwrap() += 1;
            let thread_count_rc2 = Arc::clone(&thread_count);
            let handle = thread::spawn(move || {
                let score = -_nega_alpha_concurrency(
                    next_p,
                    next_o,
                    depth - 1,
                    1,
                    -beta,
                    -alpha,
                    thread_count_rc,
                    thread_max,
                );
                scores_rc.lock().unwrap().push(score);
                *thread_count_rc2.lock().unwrap() -= 1;
            });
            handles.push(handle);
        }
        legal_patt &= !(1u64 << pos);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let scores_rc = scores.lock().unwrap();
    *scores_rc.iter().max().unwrap()
}

// #[inline]
// pub fn nega_alpha(p: u64, o: u64, depth: usize, mode: isize) -> isize {
//     return _nega_alpha(p, o, depth, mode, isize::MIN + 1, isize::MAX);
// }

use std::sync::atomic::{AtomicBool, Ordering};

pub struct Worker {
    to_stop: Arc<AtomicBool>,
}

trait WorkerTrait {
    fn run(&self);
    fn stop(&self);
}

impl Worker {
    fn new(p: u64, o: u64, depth: usize, mode: isize, alpha: isize, beta: isize) -> Self {
        Worker {
            to_stop: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl WorkerTrait for Worker {
    fn run(&self) {
        let to_stop = Arc::clone(&self.to_stop);
        thread::spawn(move || loop {
            if to_stop.load(Ordering::Relaxed) {
                break;
            }
            println!("worker1 working ");
            // thread::sleep(time::Duration::from_millis(1000)); // 擬似処理.
            // thread break?
        });
    }
    fn stop(&self) {
        self.to_stop.store(true, Ordering::Relaxed)
    }
}
