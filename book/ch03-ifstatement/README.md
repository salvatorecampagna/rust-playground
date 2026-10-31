# If Statement

The Rust Programming Language,
[chapter 3.5](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions).
Reads an integer and prints whether it's even or odd, first with an `if`
statement and then with `if` as an expression.

```bash
cargo run -p ifstatement
```

## Learnings

### #1 — `if` is an expression

The value of the chosen branch's block becomes the value of the whole `if`, so
`let x = if n % 2 == 0 { "even" } else { "odd" };` is Rust's version of Java's
`cond ? a : b`, without a separate operator.

**References:** [Rust Book — Using `if` in a `let` Statement](https://doc.rust-lang.org/book/ch03-05-control-flow.html#using-if-in-a-let-statement) ·
[Rust Reference — `if` expressions](https://doc.rust-lang.org/reference/expressions/if-expr.html)

### #2 — Both branches must have the same type

My experiment with `"even"` in one branch and `1` in the other failed with
E0308, `if` and `else` have incompatible types. The error is reported against
the first branch's type. It's commented out in `main.rs`.

**References:** [Rust Book — Using `if` in a `let` Statement](https://doc.rust-lang.org/book/ch03-05-control-flow.html#using-if-in-a-let-statement) ·
[E0308](https://doc.rust-lang.org/error_codes/E0308.html)

### #3 — An `if` without `else` has type `()`

Using an `if` without `else` as a value failed with E0317. Without `else` the
`if` has type `()`, so the `true` branch must produce `()` too.

**References:** [Rust Reference — `if` expressions](https://doc.rust-lang.org/reference/expressions/if-expr.html) ·
[E0317](https://doc.rust-lang.org/error_codes/E0317.html)

### #4 — The condition must be a `bool`

`if input` with a number failed with E0308, expected `bool`. Java requires a
`boolean` as well; C and JavaScript accept "truthy" numbers.

**References:** [Rust Book — `if` Expressions](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions) ·
[E0308](https://doc.rust-lang.org/error_codes/E0308.html)

### #5 — `%` is a remainder, not a modulo

I wasn't sure whether the odd/even check works for negative numbers. `-3 % 2`
is `-1`, in Rust and in Java, because the result takes the sign of the left
operand. Checking `n % 2 == 0` works for negative numbers; `n % 2 == 1` would
call `-3` even. For a result that's never negative, use `rem_euclid`.

**References:** [`Rem` for `i32`](https://doc.rust-lang.org/std/ops/trait.Rem.html#impl-Rem-for-i32) ·
[`i32::rem_euclid`](https://doc.rust-lang.org/std/primitive.i32.html#method.rem_euclid)

### #6 — `expect` over `unwrap`

I replaced `unwrap()` with `expect`. Both panic on an error, but
`expect("...")` says which step failed, where `unwrap()` only reports that
something did.

**References:** [Rust Book — Shortcuts for Panic on Error: `unwrap` and `expect`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#shortcuts-for-panic-on-error-unwrap-and-expect)
