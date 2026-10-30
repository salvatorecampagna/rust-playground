use fizzbuzz::*;

#[test]
fn one_is_1() {
    let input = 1;
    let output = fizzbuzz(input);
    let expected = "1";
    assert_eq!(output, expected);
}

#[test]
fn two_is_2() {
    let input = 2;
    let output = fizzbuzz(input);
    let expected = "2";
    assert_eq!(output, expected);
}

#[test]
fn three_is_fizz() {
    let input = 3;
    let output = fizzbuzz(input);
    let expected = "Fizz";
    assert_eq!(output, expected);
}

#[test]
fn five_is_buzz() {
    let input = 5;
    let output = fizzbuzz(input);
    let expected = "Buzz";
    assert_eq!(output, expected);
}

#[test]
fn fifteen_is_fizzbuzz() {
    let input = 15;
    let output = fizzbuzz(input);
    let expected = "FizzBuzz";
    assert_eq!(output, expected);
}

#[test]
fn a_larger_multiple_of_3_is_fizz() {
    let input = 99;
    let output = fizzbuzz(input);
    let expected = "Fizz";
    assert_eq!(output, expected);
}

#[test]
fn a_larger_multiple_of_5_is_buzz() {
    let input = 100;
    let output = fizzbuzz(input);
    let expected = "Buzz";
    assert_eq!(output, expected);
}

#[test]
fn a_larger_multiple_of_15_is_fizzbuzz() {
    let input = 90;
    let output = fizzbuzz(input);
    let expected = "FizzBuzz";
    assert_eq!(output, expected);
}

#[test]
fn a_number_not_divisible_by_3_or_5_is_itself() {
    let input = 98;
    let output = fizzbuzz(input);
    let expected = "98";
    assert_eq!(output, expected);
}

#[test]
fn zero_is_divisible_by_everything() {
    let input = 0;
    let output = fizzbuzz(input);
    let expected = "FizzBuzz";
    assert_eq!(output, expected);
}

#[test]
fn the_largest_u32_is_fizzbuzz() {
    // u32::MAX = 4294967295 = 2^32 - 1 = 3 * 5 * 17 * 257 * 65537
    let input = u32::MAX;
    let output = fizzbuzz(input);
    let expected = "FizzBuzz";
    assert_eq!(output, expected);
}
