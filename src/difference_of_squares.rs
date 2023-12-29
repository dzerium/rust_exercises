pub mod difference_of_squares {
    pub fn square_of_sum(n: u32) -> u32 {
        (1..=n).sum::<u32>().pow(2)
    }

    pub fn sum_of_squares(n: u32) -> u32 {
        (1..=n).fold(0, |acc, x| acc + x.pow(2))
    }

    pub fn difference(n: u32) -> u32 {
        square_of_sum(n) - sum_of_squares(n)
    }
}

use difference_of_squares as squares;
#[test]
fn square_of_sum_1() {
    assert_eq!(1, squares::square_of_sum(1));
}
#[test]
fn square_of_sum_5() {
    assert_eq!(225, squares::square_of_sum(5));
}
#[test]
fn square_of_sum_100() {
    assert_eq!(25_502_500, squares::square_of_sum(100));
}
#[test]
fn sum_of_squares_1() {
    assert_eq!(1, squares::sum_of_squares(1));
}
#[test]
fn sum_of_squares_5() {
    assert_eq!(55, squares::sum_of_squares(5));
}
#[test]
fn sum_of_squares_100() {
    assert_eq!(338_350, squares::sum_of_squares(100));
}
#[test]
fn difference_1() {
    assert_eq!(0, squares::difference(1));
}
#[test]
fn difference_5() {
    assert_eq!(170, squares::difference(5));
}
#[test]
fn difference_100() {
    assert_eq!(25_164_150, squares::difference(100));
}
