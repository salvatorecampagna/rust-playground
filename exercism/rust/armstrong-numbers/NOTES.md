# Notes: Armstrong Numbers

[Exercise](https://exercism.org/tracks/rust/exercises/armstrong-numbers) ·
[Solution](src/lib.rs)

- **Digits from a number, not a string.** `n % 10` is the last digit and
  `n /= 10` drops it: the mirror of [Luhn](../luhn/NOTES.md). The loop uses up
  `n`, so `let mut n = num;` keeps `num` for the final `sum == num`. A `u32`
  is `Copy`, so that's a free copy, like `int n = num;` in Java.
- **Bug the Exercism tests don't catch:** `3_999_999_999` panicked with
  `attempt to add with overflow`. Ten powers of up to 9¹⁰ don't fit in a `u32`.
  Debug builds panic on overflow where Java would silently wrap.
- **`checked_add` returns an `Option`.** `None` means the sum went past
  `u32::MAX`, so it can't equal `num`: `None => return false` is the answer,
  not an error. `pow` alone can't overflow (9¹⁰ = 3,486,784,401 fits).
- **`#[should_panic]` was the wrong fix.** It made the test pass by requiring
  the crash, locking the bug in. A function that answers yes or no should
  answer for every input; `should_panic` is for code meant to panic.
- **`ilog10` is one less than the digit count.** `153.ilog10()` is 2. My first
  try used it as the exponent and turned `None` (the input 0) into
  `return false`: 5 tests failed, and the "is not Armstrong" tests passed by
  accident because almost everything returned `false`.
- **A `match` arm either produces a value or leaves the function.** In
  `let exp = match ...`, `return false` never gives `exp` a value; it ends
  `is_armstrong_number`. The fix was arms that produce `1` and `log + 1`.
- **Method calls need a known type.** Removing `: u32` from `sum` gave E0689,
  `can't call method checked_add on ambiguous numeric type {integer}`.
  Inference can use later code like `sum == num`, but a method call is
  resolved where it's written.
- **`cargo clippy` alone skips the tests.** `#[cfg(test)]` code is only checked
  with `--all-targets`; that's where `assert_eq!(x, false)` got flagged
  (`assert!(!x)` instead).
- **No 10-digit Armstrong number fits in a `u32`.** The only one,
  4,679,307,774, is above `u32::MAX`, so every 10-digit input is `false`.
