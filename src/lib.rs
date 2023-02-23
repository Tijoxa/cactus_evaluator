use pyo3::prelude::*;
use std::str::Chars;
use std::cmp;
pub mod arrays;

fn card_to_repr(card: &str) -> u32 {
    let mut iterator: Chars = card.chars();
    let symbol: Option<char> = iterator.next();
    let color: Option<char>= iterator.next();

    let cdhs: u32;
    let rrrr: u32;
    let pppppp: u32;
    let bbbbbbbbbbbbb: u32;

    match symbol {
        Some('2') => {
            rrrr = 0x0 << 8;
            pppppp = arrays::PRIMES[0];
            bbbbbbbbbbbbb = 0x1 << 16
        }
        Some('3') => {
            rrrr = 0x1 << 8;
            pppppp = arrays::PRIMES[1];
            bbbbbbbbbbbbb = 0x1 << 17
        }
        Some('4') => {
            rrrr = 0x2 << 8;
            pppppp = arrays::PRIMES[2];
            bbbbbbbbbbbbb = 0x1 << 18
        }
        Some('5') => {
            rrrr = 0x3 << 8;
            pppppp = arrays::PRIMES[3];
            bbbbbbbbbbbbb = 0x1 << 19
        }
        Some('6') => {
            rrrr = 0x4 << 8;
            pppppp = arrays::PRIMES[4];
            bbbbbbbbbbbbb = 0x1 << 20
        }
        Some('7') => {
            rrrr = 0x5 << 8;
            pppppp = arrays::PRIMES[5];
            bbbbbbbbbbbbb = 0x1 << 21
        }
        Some('8') => {
            rrrr = 0x6 << 8;
            pppppp = arrays::PRIMES[6];
            bbbbbbbbbbbbb = 0x1 << 22
        }
        Some('9') => {
            rrrr = 0x7 << 8;
            pppppp = arrays::PRIMES[7];
            bbbbbbbbbbbbb = 0x1 << 23
        }
        Some('T') => {
            rrrr = 0x8 << 8;
            pppppp = arrays::PRIMES[8];
            bbbbbbbbbbbbb = 0x1 << 24
        }
        Some('J') => {
            rrrr = 0x9 << 8;
            pppppp = arrays::PRIMES[9];
            bbbbbbbbbbbbb = 0x1 << 25
        }
        Some('Q') => {
            rrrr = 0xa << 8;
            pppppp = arrays::PRIMES[10];
            bbbbbbbbbbbbb = 0x1 << 26
        }
        Some('K') => {
            rrrr = 0xb << 8;
            pppppp = arrays::PRIMES[11];
            bbbbbbbbbbbbb = 0x1 << 27
        }
        Some('A') => {
            rrrr = 0xc << 8;
            pppppp = arrays::PRIMES[12];
            bbbbbbbbbbbbb = 0x1 << 28
        }
        _ => panic!("symbol unfound")
    };

    match color {
        Some('C') => cdhs = 0x8 << 12,
        Some('D') => cdhs = 0x4 << 12,
        Some('H') => cdhs = 0x2 << 12,
        Some('S') => cdhs = 0x1 << 12,
        _ => panic!("color unfound")
    };

    cdhs + rrrr + pppppp + bbbbbbbbbbbbb
}

fn find_fast(mut u: u32) -> u32 {
    u += 0xe91aaa35;
    u ^= u >> 16;
    u += u << 8;
    u ^= u >> 4;
    let b: u32 = (u >> 8) & 0x1ff;
    let a: u32 = (u + (u << 2)) >> 19;
    let hash: u32 = arrays::HASH_ADJUST[b as usize];
    let c: u32 = a ^ hash;

    c
}

fn eval_5cards(c1: u32, c2: u32, c3: u32, c4: u32, c5: u32) -> u32 {
    let q: u32 = (c1 | c2 | c3 | c4 | c5) >> 16;

    // This checks for Flushes and Straight Flushes
    if (c1 & c2 & c3 & c4 & c5 & 0xf000) != 0 {
        return arrays::FLUSHES[q as usize];
    }

    // This checks for Straights and High Card hands
    let unique5: u32 = arrays::UNIQUE_5[q as usize];
    if unique5 != 0 {
        return unique5;
    }

    // This performs a perfect-hash lookup for remaining hands
    let q2: u32 = (c1 & 0xff) * (c2 & 0xff) * (c3 & 0xff) * (c4 & 0xff) * (c5 & 0xff);
    let result: u32 = arrays::HASH_VALUES[find_fast(q2) as usize];

    result
}

#[pyfunction]
fn evaluate(card1: &str, card2: &str, card3: &str, card4: &str, card5: &str) -> PyResult<u32> {
    let c1: u32 = card_to_repr(card1);
    let c2: u32 = card_to_repr(card2);
    let c3: u32 = card_to_repr(card3);
    let c4: u32 = card_to_repr(card4);
    let c5: u32 = card_to_repr(card5);

    let result: u32 = eval_5cards(c1, c2, c3, c4, c5);
    
    Ok(result)
}

#[pyfunction]
fn evaluate_7(card1: &str, card2: &str, card3: &str, card4: &str, card5: &str, card6: &str, card7: &str) -> PyResult<u32> {
    let c1: u32 = card_to_repr(card1);
    let c2: u32 = card_to_repr(card2);
    let c3: u32 = card_to_repr(card3);
    let c4: u32 = card_to_repr(card4);
    let c5: u32 = card_to_repr(card5);
    let c6: u32 = card_to_repr(card6);
    let c7: u32 = card_to_repr(card7);

    let cards: [u32; 7] = [c1, c2, c3, c4, c5, c6, c7];

    let mut result: u32 = 7462;

    for [id1, id2, id3, id4, id5] in arrays::PERM7 {
        let tmp_result: u32 = eval_5cards(cards[*id1], cards[*id2], cards[*id3], cards[*id4], cards[*id5]);
        result = cmp::min(tmp_result, result);
    }
    
    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn cactus_evaluator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(evaluate, m)?)?;
    m.add_function(wrap_pyfunction!(evaluate_7, m)?)?;
    Ok(())
}