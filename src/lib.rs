// Ce script est une adaptation en package Rust de l'algorithme de Cactus Kev Perfect Hash
// qui permet d'évaluer une main de poker en lui donnant un score de 1 (meilleur) à 7462 (pire)
// https://suffe.cool/poker/evaluator.html

// +--------+--------+--------+--------+
// |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
// +--------+--------+--------+--------+

#![allow(clippy::needless_late_init)]

use pyo3::prelude::*;
use std::cmp;
mod arrays;
mod card_and_repr;
mod tools;
use crate::card_and_repr::*;

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
    let result: u32 = arrays::HASH_VALUES[tools::find_fast(q2) as usize];

    result
}

#[pyfunction]
pub fn evaluate(card1: &str, card2: &str, card3: &str, card4: &str, card5: &str) -> PyResult<u32> {
    let c1: u32 = card_to_repr(card1);
    let c2: u32 = card_to_repr(card2);
    let c3: u32 = card_to_repr(card3);
    let c4: u32 = card_to_repr(card4);
    let c5: u32 = card_to_repr(card5);

    let result: u32 = eval_5cards(c1, c2, c3, c4, c5);

    Ok(result)
}

#[pyfunction]
pub fn evaluate_7(card1: &str, card2: &str, card3: &str, card4: &str, card5: &str, card6: &str, card7: &str) -> PyResult<u32> {
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
        let tmp_result: u32 = eval_5cards(
            cards[*id1],
            cards[*id2],
            cards[*id3],
            cards[*id4],
            cards[*id5],
        );
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

// Tests of lib
#[cfg(test)]
mod lib_tests {
    use super::eval_5cards;

    #[test]
    fn eval_5cards_tests() {
        let res_1 = eval_5cards(8398611, 4204049, 2106637, 1057803, 16787479);
        assert_eq!(res_1, 5);

        let res_2 = eval_5cards(268471337, 295429, 557831, 16812055, 268446761);
        assert_eq!(res_2, 3484);
    }
}
