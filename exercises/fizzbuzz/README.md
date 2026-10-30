# FizzBuzz

A warm-up for Exercism's Raindrops exercise, in the same style.

## Instructions

Convert a number into its FizzBuzz word.

If a given number:

- is divisible by both 3 and 5, the result is `"FizzBuzz"`.
- is divisible by 3 (but not 5), the result is `"Fizz"`.
- is divisible by 5 (but not 3), the result is `"Buzz"`.
- is **not** divisible by 3 or 5, the result is the number as a string.

## Examples

- 9 is divisible by 3, so the result is `"Fizz"`.
- 10 is divisible by 5, so the result is `"Buzz"`.
- 30 is divisible by 3 and 5, so the result is `"FizzBuzz"`.
- 7 is not divisible by 3 or 5, so the result is `"7"`.

## Running the tests

All but the first test are ignored. Remove the `#[ignore]` line from the next
test in `tests/fizzbuzz.rs` once the current one passes, or run everything with:

```bash
cargo test -p fizzbuzz -- --include-ignored
```
