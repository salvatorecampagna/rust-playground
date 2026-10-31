# Arrays

The Rust Programming Language,
[chapter 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type).
Prints arrays in their three literal forms and looks up a weekday by an index
read from stdin.

```bash
cargo run -p arrays
```

## Notes

- **Three ways to write an array:** inferred (`["Sunday", ...]` is a
  `[&str; 7]`), annotated (`[u32; 3]`), and repeated (`[100; 7]` is seven
  `100`s). The length is part of the type, so `[u32; 3]` and `[u32; 4]` are
  different types; that's why functions take a slice `&[T]` instead.
- **Every index is bounds-checked.** Entering `10` panics at run time with
  `index out of bounds: the len is 7 but the index is 10`. A bad index never
  reads past the array, like Java's `ArrayIndexOutOfBoundsException`.
- **A constant index is caught at compile time.** The commented-out
  `weekdays[10]` line doesn't compile: `this operation will panic at runtime`.
  That's the `unconditional_panic` lint (an error by default) spotting that
  `10 >= 7`, not the type system. With an index from input the compiler can't
  know, so the check happens when the program runs.
- **`-1` fails before it reaches the array.** The index is a `usize`, which is
  unsigned, so parsing `-1` gives `InvalidDigit`, the same surprise as `"-5"`
  as `u32` in [`ch03-infertype`](../ch03-infertype). Pressing Enter gives
  `Empty`.
