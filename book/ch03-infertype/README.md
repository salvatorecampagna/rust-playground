# Infer Type

The Rust Programming Language,
[chapter 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html).
Parses a few strings as `i32` and prints the value or the error.

```bash
cargo run -p infertype
```

## Learnings

### #1 — `"42".parse()` alone doesn't compile

I got E0284. The signature is
`parse<F: FromStr>(&self) -> Result<F, F::Err>`, and `F` appears only in the
return type, so nothing tells the compiler what to build. Fixes:
`let x: Result<i32, _> = ...` or `"42".parse::<i32>()`. The error type can
stay `_` because it follows from `F`.

**References:** [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) ·
[E0284](https://doc.rust-lang.org/error_codes/E0284.html)

### #2 — `let x: i32 = "42".parse();` is wrong too

`x` holds the `Result`, not the number. It only worked with `.expect(...)`,
which unwraps it.

**References:** [Rust Book — Recoverable Errors with `Result`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#recoverable-errors-with-result)

### #3 — `"-5"` as `u32` gives `InvalidDigit`, not a "negative" error

I expected a negative-number error. The parser in `core/src/num/mod.rs`
strips a leading `-` only for signed types, so for `u32` the `-` is just a bad
digit. Java's `Integer.parseUnsignedInt("-5")` says "Illegal leading minus
sign" instead.

**References:** [`u32::from_str_radix`](https://doc.rust-lang.org/std/primitive.u32.html#method.from_str_radix) ·
[`IntErrorKind::InvalidDigit`](https://doc.rust-lang.org/std/num/enum.IntErrorKind.html#variant.InvalidDigit)

### #4 — `i32::Err` doesn't work

I tried `i32::Err` to name the error type. `Err` belongs to the `FromStr`
impl, so the full name is `<i32 as FromStr>::Err`. A faster way to find a
type: write `let () = expr;` and read the compiler error.

**References:** [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) ·
[Rust Reference — Qualified paths](https://doc.rust-lang.org/reference/paths.html#qualified-paths)

### #5 — Errors are values

`parse` returns a `Result` that has to be handled, where Java's
`Integer.parseInt` throws an unchecked `NumberFormatException` that's easy to
forget.

**References:** [Rust Book — Recoverable Errors with `Result`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#recoverable-errors-with-result)
