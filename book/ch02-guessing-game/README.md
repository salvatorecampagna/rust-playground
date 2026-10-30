# Guessing Game

The Rust Programming Language,
[chapter 2](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).

```bash
cargo run -p guessing-game
```

## Notes

- **The book's code didn't compile.** The book targets `rand` 0.8, and this
  project uses 0.10, where `thread_rng()` became `rand::rng()`, `gen_range`
  became `random_range`, and the trait is `RngExt`. `cargo doc -p rand --open`
  documents the exact version in `Cargo.lock`, which beats the book or a web
  search when names change.
- **A range can't be printed with `{}`.** `RangeInclusive` implements `Debug`
  but not `Display`, so it needs `{:?}`. Rust splits Java's `toString()` into a
  user-facing trait and a developer-facing one.
- **Passing the range to `random_range` used it up.** The parameter is `R`, not
  `&R`, and ranges aren't `Copy`, so the call moves the range. Using it
  afterwards is "borrow of moved value". `.clone()` is the fix, and it's cheap
  for two integers. In Java the method would just get another reference to the
  same object.
