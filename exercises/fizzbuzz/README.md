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

The exercise starts with every test but the first marked `#[ignore]`; remove
the next `#[ignore]` in `tests/fizzbuzz.rs` each time the current test passes.
This solution is complete, so all tests are enabled:

```bash
cargo test -p fizzbuzz
```

## Learnings

### #1 — A `match` on `(n % 3, n % 5)` fits because exactly one case wins

`(0, 0)` has to come first, since 15 also matches `(0, _)` and `(_, 0)`. Arms
are tried top to bottom and the first match wins.

**References:** [Rust Reference — `match` expressions](https://doc.rust-lang.org/reference/expressions/match-expr.html) ·
[Rust Reference — Tuple patterns](https://doc.rust-lang.org/reference/patterns.html#tuple-patterns)

### #2 — `"Fizz".to_string()` allocates

I asked whether `to_string` returns a copy, and it does: a new, owned
`String`. The literal is a `&str` borrowed from the program binary, and the
function promises an owned `String`. Since `n.to_string()` must be a `String`,
every arm has to produce one. In Java, literals and computed strings are both
just `String`.

**References:** [Rust Book — String Literals as Slices](https://doc.rust-lang.org/book/ch04-03-slices.html#string-literals-as-slices) ·
[`ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html)

### #3 — Two tests surprised me

0 is `"FizzBuzz"` (divisible by everything), and so is `u32::MAX`, since
2³² − 1 = 3 × 5 × 17 × 257 × 65537.

**References:** [`u32::MAX`](https://doc.rust-lang.org/std/primitive.u32.html#associatedconstant.MAX)
