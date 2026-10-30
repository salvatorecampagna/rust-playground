# Infer Type

The Rust Programming Language,
[chapter 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html).
Parses a few strings as `i32` and prints the value or the error.

```bash
cargo run -p infertype
```

## Notes

- **`"42".parse()` alone doesn't compile.** The signature is
  `parse<F: FromStr>(&self) -> Result<F, F::Err>`, and `F` appears only in the
  return type, so nothing tells the compiler what to build (E0284). Fixes:
  `let x: Result<i32, _> = ...` or `"42".parse::<i32>()`. The error type can
  stay `_` because it follows from `F`.
- **`let x: i32 = "42".parse();` is wrong too.** `x` holds the `Result`, not
  the number. It only worked with `.expect(...)`, which unwraps it.
- **`"-5"` as `u32` gives `InvalidDigit`, not a "negative" error.** The parser
  in `core/src/num/mod.rs` strips a leading `-` only for signed types, so for
  `u32` the `-` is just a bad digit. Java's `Integer.parseUnsignedInt("-5")`
  says "Illegal leading minus sign" instead.
- **`i32::Err` doesn't work.** `Err` belongs to the `FromStr` impl, so the full
  name is `<i32 as FromStr>::Err`. A faster way to find a type: write
  `let () = expr;` and read the compiler error.
- **Errors are values.** `parse` returns a `Result` that has to be handled, where
  Java's `Integer.parseInt` throws an unchecked `NumberFormatException` that's
  easy to forget.
