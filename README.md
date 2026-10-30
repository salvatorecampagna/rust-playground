# 🦀 Rust Playground

My workspace for learning Rust: exercises from
[*The Rust Programming Language*](https://doc.rust-lang.org/book/) ("the book"),
the [Exercism Rust track](https://exercism.org/tracks/rust), and a few
custom exercises written in the same style.

## Layout

```
.
├── Cargo.toml          # Cargo workspace: book + custom exercises
├── guessing-game/      # the book, chapter 2
├── mutable/ ...        # the book, chapter 3
├── fizzbuzz/ ...       # custom Exercism-style exercises
└── rust/               # Exercism exercises (standalone, excluded from the workspace)
```

Book and custom exercises are members of one
[Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html),
so they share a single `Cargo.lock` and `target/`. Exercism downloads each
exercise as a standalone project under `rust/`, which the workspace excludes.

## The book

| Chapter | Project | What it explores |
|---|---|---|
| [2. Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) | [`guessing-game`](guessing-game/src/main.rs) | `rand`, `loop`, `match` on `Ordering`, moves vs `Clone` |
| [3.1 Variables](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) | [`mutable`](mutable/src/main.rs) | `let` vs `let mut` |
| [3.1 Variables](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) | [`shadow`](shadow/src/main.rs) | shadowing and inner scopes |
| [3.1 Variables](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) | [`variable`](variable/src/main.rs) | shadowing with a type change (`&str` → `usize`) |
| [3.2 Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html) | [`infertype`](infertype/src/main.rs) | type inference, `parse`, `Result`, `ParseIntError` |

```bash
cargo run -p guessing-game
```

## Exercism

| Exercise | Solution | What it explores | Status |
|---|---|---|---|
| [Reverse String](https://exercism.org/tracks/rust/exercises/reverse-string) | [`lib.rs`](rust/reverse-string/src/lib.rs) | `chars` vs grapheme clusters, external crates, traits adding methods | ✅ incl. bonus |
| [Leap](https://exercism.org/tracks/rust/exercises/leap) | [`lib.rs`](rust/leap/src/lib.rs) | tuple patterns, `_`, arm order | ✅ |
| [Raindrops](https://exercism.org/tracks/rust/exercises/raindrops) | [`lib.rs`](rust/raindrops/src/lib.rs) | rules as data, slices `&[T]`, `String` building | ✅ |

```bash
cd rust/raindrops
cargo test -- --include-ignored
```

## Custom exercises

Exercism-style (README, `todo!` stub, tests ignored one at a time) but not
part of the Exercism track.

| Exercise | What it explores | Status |
|---|---|---|
| [`fizzbuzz`](fizzbuzz/README.md) | warm-up for Raindrops: `match` on a tuple | ✅ |
| [`raindrops-plung`](raindrops-plung/README.md) | Raindrops with a 4th sound: does the design scale? | 🚧 |

```bash
cargo test -p fizzbuzz -- --include-ignored
```

## Notes to self

**Ownership in four lines**

| Parameter | Access | Ownership |
|---|---|---|
| `&x` | read | stays with the caller |
| `&mut x` | read + write | stays with the caller |
| `x` (`Copy`) | full | the function gets its own copy |
| `x` (not `Copy`) | full | **moves** to the function |

**Habits**

- Read compiler errors top to bottom: labels show *why*, `help:` shows the fix.
- `cargo fmt` and `cargo clippy` before every commit.
- `cargo doc -p <crate> --open` documents the exact dependency versions in use.
- Unused-variable warnings are often real bugs.
- `match` arms: most specific first. `_` only means "not 0" because earlier arms caught the zeros.
