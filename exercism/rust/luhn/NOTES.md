# Notes: Luhn

[Exercise](https://exercism.org/tracks/rust/exercises/luhn) ·
[Solution](src/lib.rs)

- **`ch as u8` is the ASCII code, not the digit.** `'7' as u8` is 55, so
  doubling it printed values like `c = 43`. `ch as u8 - b'0'` gives 7, and
  `ch.to_digit(10)` gives `Some(7)` (or `None` for a non-digit).
- **Count digits, not characters.** `enumerate()` counts from 0 and counts
  spaces too, so a space shifts which digits are "every second one". A counter
  that only moves on digits fixes it: from 0, the digits to double are at odd
  counts.
- **Bug I hit:** I computed the doubled value into `c` and then added `ch` to
  the sum. The compiler had warned `unused variable: c`, pointing right at it.
- **Skipping is not rejecting.** Skipping every non-digit made `"059a"` valid.
  Only spaces may be skipped; anything else means `return false`, which exits
  the whole function from inside the loop.
- **Parameters are immutable, like `final` in Java.** A `u32` argument is
  already a copy; `mut value` only allows changing that copy. Computing
  `let doubled = value * 2;` and returning an `if`/`else` needs no `mut` at all.
- **Panics I met:** `.unwrap()` on `' '.to_digit(10)` (`None`), and
  `' ' as u8 - b'0'`, which is `32 - 48`: debug builds panic on overflow where
  Java would silently wrap.
- **Final shape:** one `match` per character with three arms: `' '` (skip),
  `'0'..='9'` (double, add, count), `_` (invalid). A range starting at `'1'`
  sent every `0` to the invalid arm. "At least two digits" is checked on the
  counter after the loop, instead of `len()`, which counts bytes and spaces.
- **Testing a private helper:** files in `tests/` are a separate crate and
  only see `pub` items. A `#[cfg(test)] mod tests` in `lib.rs` can test private
  functions, but still needs `use super::*;` because a module has its own names.
- **Seeing `println!` output:** `cargo test <name> -- --include-ignored
  --nocapture`. A test that returns before the `println!` prints nothing.
