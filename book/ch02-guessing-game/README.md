# Guessing Game

The Rust Programming Language,
[chapter 2](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).

```bash
cargo run -p guessing-game
```

## Learnings

### #1 — The book's `rand` code didn't compile with `rand` 0.10

The book targets `rand` 0.8, and I got errors because `thread_rng()` wasn't
there. In 0.10 it became `rand::rng()`, `gen_range` became `random_range`, and
the trait to import is `RngExt`.

**References:** [`rand::rng`](https://docs.rs/rand/0.10.3/rand/fn.rng.html) ·
[`RngExt::random_range`](https://docs.rs/rand/0.10.3/rand/trait.RngExt.html#method.random_range) ·
[Rust Book — Generating a Random Number](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#generating-a-random-number)

### #2 — `cargo doc --open` didn't show the `rand` docs I expected

I couldn't find `rand` in the docs `cargo doc --open` opened.
`cargo doc -p rand --open` opens that crate directly, and it documents the
exact version in `Cargo.lock`, which beats the book or a web search when names
change between versions.

**References:** [The Cargo Book — `cargo doc`](https://doc.rust-lang.org/cargo/commands/cargo-doc.html)

### #3 — A range can't be printed with `{}`

I wanted something like Java's `toString()` to print the range.
`RangeInclusive` implements `Debug` but not `Display`, so it needs `{:?}`. Rust
splits `toString()` into a user-facing trait (`Display`) and a developer-facing
one (`Debug`).

**References:** [`std::fmt` — `Display` vs `Debug`](https://doc.rust-lang.org/std/fmt/index.html#fmtdisplay-vs-fmtdebug) ·
[`RangeInclusive`](https://doc.rust-lang.org/std/ops/struct.RangeInclusive.html)

### #4 — Passing the range to `random_range` used it up

Using the range after `random_range(valid_range)` gave "borrow of moved
value". The parameter is `R`, not `&R`, and ranges aren't `Copy`, so the call
moves the range into the function. In Java the method would just get another
reference to the same object. I fixed it with `.clone()`, which is cheap here:
the range is only two integers.

**References:** [Rust Book — Variables and Data Interacting with Move](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move) ·
[Rust Book — Variables and Data Interacting with Clone](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-clone) ·
[E0382](https://doc.rust-lang.org/error_codes/E0382.html)
