# Arrays

The Rust Programming Language,
[chapter 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type).
Prints arrays in their three literal forms and looks up a weekday by an index
read from stdin.

```bash
cargo run -p arrays
```

## Learnings

### #1 — The length is part of an array's type

There are three ways to write an array: inferred (`["Sunday", ...]` is a
`[&str; 7]`), annotated (`[u32; 3]`), and repeated (`[100; 7]` is seven
`100`s). Because the length is in the type, `[u32; 3]` and `[u32; 4]` are
different types, which is why functions take a slice `&[T]` instead.

**References:** [Rust Book — The Array Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type) ·
[Rust Reference — Array types](https://doc.rust-lang.org/reference/types/array.html)

### #2 — Every index is checked at run time

Entering `10` panics with
`index out of bounds: the len is 7 but the index is 10`. A bad index never
reads past the array, like Java's `ArrayIndexOutOfBoundsException`.

**References:** [Rust Book — Invalid Array Element Access](https://doc.rust-lang.org/book/ch03-02-data-types.html#invalid-array-element-access)

### #3 — `.get()` returns `None` instead of panicking

With index `10`, `weekdays.get(10)` returns `None`, so the program prints
`day: "none"` and only then panics on `weekdays[10]`. `.get()` returns
`Option<&&str>`, a reference to the element, so the default passed to
`unwrap_or` must be a reference too: `unwrap_or(&"none")`.

**References:** [`slice::get`](https://doc.rust-lang.org/std/primitive.slice.html#method.get) ·
[`Option::unwrap_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or)

### #4 — A constant out-of-bounds index doesn't compile

The commented-out `weekdays[10]` line fails with
`this operation will panic at runtime`. That's the `unconditional_panic` lint
(an error by default) spotting that `10 >= 7`, not the type system. With an
index from input the compiler can't know, so the check happens when the
program runs.

**References:** [rustc lints — `unconditional_panic`](https://doc.rust-lang.org/rustc/lints/listing/deny-by-default.html#unconditional-panic)

### #5 — `-1` fails before it reaches the array

The index is a `usize`, which is unsigned, so parsing `-1` gives
`InvalidDigit`, the same surprise as `"-5"` as `u32` in
[`ch03-infertype`](../ch03-infertype). Pressing Enter gives `Empty`.

**References:** [`IntErrorKind`](https://doc.rust-lang.org/std/num/enum.IntErrorKind.html#variant.InvalidDigit) ·
[`u32::from_str_radix`](https://doc.rust-lang.org/std/primitive.u32.html#method.from_str_radix)
