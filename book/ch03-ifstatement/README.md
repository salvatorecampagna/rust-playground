# If Statement

The Rust Programming Language,
[chapter 3.5](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions).
Reads an integer and prints whether it's even or odd, first with an `if`
statement and then with `if` as an expression.

```bash
cargo run -p ifstatement
```

## Notes

- **`if` is an expression.** The value of the chosen branch's block becomes the
  value of the whole `if`, so `let x = if n % 2 == 0 { "even" } else { "odd" };`
  is Rust's version of Java's `cond ? a : b`, without a separate operator.
- **Three experiments that don't compile** (commented out in `main.rs`, each
  with its error):
  - branches of different types (`"even"` vs `1`): E0308, `if` and `else` have
    incompatible types. The error is reported against the first branch's type.
  - an `if` without `else` used as a value: E0317. Without `else` the `if` has
    type `()`, so the `true` branch must produce `()` too.
  - a number as the condition (`if input`): E0308, expected `bool`. Java
    requires a `boolean` as well; C and JavaScript accept "truthy" numbers.
- **`%` is a remainder, not a modulo.** `-3 % 2` is `-1`, in Rust and in Java,
  because the result takes the sign of the left operand. Checking `n % 2 == 0`
  works for negative numbers; `n % 2 == 1` would call `-3` even. For a result
  that's never negative, use `rem_euclid`.
- **`expect` over `unwrap`.** Both panic on an error, but `expect("...")` says
  which step failed, where `unwrap()` only reports that something did.
