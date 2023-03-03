// Tests of what are on the crate interface
use cactus_evaluator;

#[test]
fn evaluate_test() {
    let evaluation = cactus_evaluator::evaluate(&"9S", &"8H", &"AS", &"6H", &"TH");
    match evaluation {
        Ok(x) => assert_eq!(x, 6555),
        _ => assert!(false),
    }
}

#[test]
fn evaluate_7_test() {
    let evaluation_7 = cactus_evaluator::evaluate_7(&"TC", &"8H", &"AS", &"6H", &"TH", &"2D", &"3C");
    match evaluation_7 {
        Ok(x) => assert_eq!(x, 4241),
        _ => assert!(false),
    }
}
