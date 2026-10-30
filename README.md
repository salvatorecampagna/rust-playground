# Rust Playground

[![CI](https://github.com/salvatorecampagna/rust-playground/actions/workflows/ci.yml/badge.svg)](https://github.com/salvatorecampagna/rust-playground/actions/workflows/ci.yml)

Learning Rust, coming from Java: examples from
[*The Rust Programming Language*](https://doc.rust-lang.org/book/), exercises
from the [Exercism Rust track](https://exercism.org/tracks/rust), and a few
custom exercises in the same style. Each exercise has notes on what surprised
me and why the compiler behaved the way it did. Ideas that span several
exercises are in [NOTES.md](NOTES.md).

## Exercises

### The book

| Chapter | Project | Topics |
|---|---|---|
| [2](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) | [Guessing Game](book/ch02-guessing-game) | `rand` 0.10, `match` on `Ordering`, `Display` vs `Debug`, moves vs `Clone` |
| [3.1](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) | [Mutable](book/ch03-mutable) | `let` vs `let mut` |
| [3.1](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing) | [Shadow](book/ch03-shadow) | shadowing, inner scopes |
| [3.1](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing) | [Variable](book/ch03-variable) | shadowing with a type change |
| [3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html) | [Infer Type](book/ch03-infertype) | `parse` and inference, `Result`, `ParseIntError` |

### Exercism

My solutions to exercises from the [Rust track](https://exercism.org/tracks/rust)
([profile](https://exercism.org/profiles/salvatorecampagna)).

| Exercise | Solution | Topics |
|---|---|---|
| [Reverse String](https://exercism.org/tracks/rust/exercises/reverse-string) | [code](exercism/rust/reverse-string/src/lib.rs) · [notes](exercism/rust/reverse-string/NOTES.md) | bytes vs `char`s vs graphemes, external crates, trait methods |
| [Leap](https://exercism.org/tracks/rust/exercises/leap) | [code](exercism/rust/leap/src/lib.rs) · [notes](exercism/rust/leap/NOTES.md) | tuple patterns, `_`, arm order |
| [Raindrops](https://exercism.org/tracks/rust/exercises/raindrops) | [code](exercism/rust/raindrops/src/lib.rs) · [notes](exercism/rust/raindrops/NOTES.md) | rules as data, slices, building a `String` |

### Custom

Written in the Exercism style: a README, a `todo!()` stub, and tests that start
out `#[ignore]`d and are enabled one at a time.

| Exercise | Topics |
|---|---|
| [FizzBuzz](exercises/fizzbuzz) | warm-up for Raindrops: `match` on a tuple |
| [Raindrops: Plung](exercises/raindrops-plung) *(in progress)* | does the Raindrops design scale to a fourth rule? |

## Running

```bash
cargo run -p guessing-game                    # a book project
cargo test -p fizzbuzz                        # a custom exercise: enabled tests
cargo test -p fizzbuzz -- --include-ignored   # ...or every test
cargo test --workspace                        # all book and custom projects
cd exercism/rust/leap && cargo test           # an Exercism exercise (standalone)
```

`cargo test --workspace` fails on unfinished exercises (their `todo!()` panics).
CI skips them automatically.

## Layout

```
book/        # The Rust Programming Language, one crate per example (chNN-name)
exercises/   # custom Exercism-style exercises
exercism/    # Exercism CLI workspace; exercises land in exercism/rust/<slug>
NOTES.md     # notes spanning several exercises
```

`book/` and `exercises/` form one Cargo workspace (`members = ["book/*",
"exercises/*"]`), so they share a `Cargo.lock` and `target/`. Exercism
exercises are standalone projects, excluded from the workspace, so they stay
exactly as the Exercism CLI downloads and submits them.

To add something:

- **Book example:** `cargo new book/ch04-<name>`. The workspace picks it up.
- **Exercism exercise:** `exercism download --track=rust --exercise=<slug>`,
  then `exercism submit src/lib.rs` from its folder.
- **Custom exercise:** a library crate in `exercises/` with a `README.md`, a
  `todo!()` stub in `src/lib.rs`, and tests in `tests/`.

## CI

Each push runs `cargo fmt --check`, `cargo clippy -D warnings`, and the tests
for every project. An exercise whose `src/` still contains `todo!()` counts as
unfinished: it must build and pass lints, but its tests are skipped. Elsewhere
only enabled tests run, so an exercise can be pushed halfway through.
