# Notes: Armstrong Numbers

[Exercise](https://exercism.org/tracks/rust/exercises/armstrong-numbers) ·
[Solution](src/lib.rs)

## Learnings

### #1 — Digits from a number, not a string

`n % 10` is the last digit and `n /= 10` drops it: the mirror of
[Luhn](../luhn/NOTES.md). The loop uses up `n`, so `let mut n = num;` keeps
`num` for the final `sum == num`. A `u32` is `Copy`, so that's a free copy,
like `int n = num;` in Java.

**References:** [Rust Book — Stack-Only Data: Copy](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy) ·
[`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html)

### #2 — A bug the Exercism tests don't catch

`3_999_999_999` panicked with `attempt to add with overflow`. Ten powers of up
to 9¹⁰ don't fit in a `u32`. The panic comes from the overflow checks that
debug builds enable; see [Luhn #6](../luhn/NOTES.md) for release builds.

**References:** [Rust Book — Integer Overflow](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow) ·
[Rust Reference — Overflow](https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow)

### #3 — `checked_add` turns overflow into an answer

`checked_add` returns an `Option`. `None` means the sum went past `u32::MAX`,
so it can't equal `num`: `None => return false` is the answer, not an error.
`pow` alone can't overflow (9¹⁰ = 3,486,784,401 fits).

**References:** [`u32::checked_add`](https://doc.rust-lang.org/std/primitive.u32.html#method.checked_add) ·
[`u32::pow`](https://doc.rust-lang.org/std/primitive.u32.html#method.pow)

### #4 — `#[should_panic]` was the wrong fix

It made the test pass by requiring the crash, locking the bug in. A function
that answers yes or no should answer for every input; `should_panic` is for
code meant to panic.

**References:** [Rust Book — Checking for Panics with `should_panic`](https://doc.rust-lang.org/book/ch11-01-writing-tests.html#checking-for-panics-with-should_panic)

### #5 — `ilog10` is one less than the digit count

`153.ilog10()` is 2. My first try used it as the exponent and turned `None`
(the input 0) into `return false`: 5 tests failed, and the "is not Armstrong"
tests passed by accident because almost everything returned `false`.

**References:** [`u32::ilog10`](https://doc.rust-lang.org/std/primitive.u32.html#method.ilog10) ·
[`u32::checked_ilog10`](https://doc.rust-lang.org/std/primitive.u32.html#method.checked_ilog10)

### #6 — A `match` arm either produces a value or leaves the function

In `let exp = match ...`, `return false` never gives `exp` a value; it ends
`is_armstrong_number`. The fix was arms that produce `1` and `log + 1`.

**References:** [Rust Book — Matching with `Option<T>`](https://doc.rust-lang.org/book/ch06-02-match.html#matching-with-optiont) ·
[Rust Reference — `return` expressions](https://doc.rust-lang.org/reference/expressions/return-expr.html)

### #7 — Method calls need a known type

Removing `: u32` from `sum` gave E0689,
`can't call method checked_add on ambiguous numeric type {integer}`.
Inference can use later code like `sum == num`, but a method call is resolved
where it's written.

**References:** [E0689](https://doc.rust-lang.org/error_codes/E0689.html)

### #8 — `cargo clippy` alone skips the tests

`#[cfg(test)]` code is only checked with `--all-targets`; that's where
`assert_eq!(x, false)` got flagged (`assert!(!x)` instead).

**References:** [The Cargo Book — Target Selection](https://doc.rust-lang.org/cargo/commands/cargo-check.html#target-selection) ·
[Rust Book — The Tests Module and `#[cfg(test)]`](https://doc.rust-lang.org/book/ch11-03-test-organization.html#the-tests-module-and-cfgtest)

### #9 — No 10-digit Armstrong number fits in a `u32`

The only one, 4,679,307,774, is above `u32::MAX` (4,294,967,295), so every
10-digit input is `false`. The list of all Armstrong numbers comes from OEIS,
not from anything I computed.

**References:** [OEIS A005188 — Armstrong numbers](https://oeis.org/A005188) ·
[`u32::MAX`](https://doc.rust-lang.org/std/primitive.u32.html#associatedconstant.MAX)
