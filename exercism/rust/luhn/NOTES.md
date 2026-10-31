# Notes: Luhn

[Exercise](https://exercism.org/tracks/rust/exercises/luhn) ·
[Solution](src/lib.rs)

## Learnings

### #1 — `ch as u8` is the ASCII code, not the digit

`'7' as u8` is 55, so my doubling printed values like `c = 43`.
`ch as u8 - b'0'` gives 7, and `ch.to_digit(10)` gives `Some(7)` (or `None`
for a non-digit).

**References:** [Rust Reference — Type cast expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions) ·
[Rust Reference — Byte literals](https://doc.rust-lang.org/reference/tokens.html#byte-literals) ·
[`char::to_digit`](https://doc.rust-lang.org/std/primitive.char.html#method.to_digit)

### #2 — Count digits, not characters

`enumerate()` counts from 0 and counts spaces too, so a space shifts which
digits are "every second one". A counter that only moves on digits fixes it:
from 0, the digits to double are at odd counts.

**References:** [`Iterator::enumerate`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate)

### #3 — The unused-variable warning pointed at my bug

I computed the doubled value into `c` and then added `ch` to the sum. The
compiler had warned `unused variable: c`, pointing right at it.

**References:** [rustc lints — `unused_variables`](https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#unused-variables)

### #4 — Skipping is not rejecting

Skipping every non-digit made `"059a"` valid. Only spaces may be skipped;
anything else means `return false`, which exits the whole function from
inside the loop.

**References:** [Rust Reference — `return` expressions](https://doc.rust-lang.org/reference/expressions/return-expr.html)

### #5 — Parameters are immutable, like `final` in Java

A `u32` argument is already a copy; `mut value` only allows changing that
copy. Computing `let doubled = value * 2;` and returning an `if`/`else` needs
no `mut` at all.

**References:** [Rust Reference — Function parameters](https://doc.rust-lang.org/reference/items/functions.html#function-parameters) ·
[Rust Book — Functions with Return Values](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#functions-with-return-values)

### #6 — Two panics I met

`.unwrap()` on `' '.to_digit(10)` (`None`), and `' ' as u8 - b'0'`, which is
`32 - 48`: debug builds panic on overflow where Java would silently wrap.

**References:** [`Option::unwrap`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap) ·
[Rust Book — Integer Overflow](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow)

### #7 — One `match` with three arms

The final shape is one `match` per character: `' '` (skip), `'0'..='9'`
(double, add, count), `_` (invalid). A range starting at `'1'` sent every `0`
to the invalid arm.

**References:** [Rust Book — Matching Ranges of Values with `..=`](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#matching-ranges-of-values-with-) ·
[Rust Reference — Range patterns](https://doc.rust-lang.org/reference/patterns.html#range-patterns)

### #8 — Checking the length on the digit counter

"At least two digits" is checked on the counter after the loop, instead of
`len()`, which counts bytes and spaces.

**References:** [`str::len`](https://doc.rust-lang.org/std/primitive.str.html#method.len)

### #9 — Testing a private helper

Files in `tests/` are a separate crate and only see `pub` items. A
`#[cfg(test)] mod tests` in `lib.rs` can test private functions, but still
needs `use super::*;` because a module has its own names.

**References:** [Rust Book — Testing Private Functions](https://doc.rust-lang.org/book/ch11-03-test-organization.html#testing-private-functions) ·
[Rust Book — Integration Tests](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests) ·
[Rust Book — Starting Relative Paths with `super`](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#starting-relative-paths-with-super)

### #10 — Seeing `println!` output in tests

`cargo test <name> -- --include-ignored --nocapture`. A test that returns
before the `println!` prints nothing.

**References:** [Rust Book — Showing Function Output](https://doc.rust-lang.org/book/ch11-02-running-tests.html#showing-function-output) ·
[Rust Book — Running a Subset of Tests by Name](https://doc.rust-lang.org/book/ch11-02-running-tests.html#running-a-subset-of-tests-by-name)
