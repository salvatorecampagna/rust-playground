# Notes

Ideas that came up across several exercises. Notes on a single exercise live
next to it (its `README.md`, or `NOTES.md` for Exercism exercises).

## Ownership

| Parameter | Access | Ownership |
|---|---|---|
| `&x` | read | stays with the caller |
| `&mut x` | read + write | stays with the caller |
| `x` (`Copy`) | full | the function gets its own copy |
| `x` (not `Copy`) | full | moves to the function |

- Rust has no value-type vs reference-type split like Java. Every type is a
  value; `Copy` decides whether passing it duplicates or moves.
- `Copy` is implicit and cheap (`u32`, `bool`, `&str`). `Clone` is an explicit
  `.clone()`, which can mean copying a heap buffer.
- A `String` is a pointer, length and capacity. Moving it copies those three
  fields, never the text.
- Ask for the least access needed: `&T` first, `&mut T` to modify, `T` only to
  keep or consume. Prefer `&str` over `&String` and `&[T]` over `&Vec<T>`.

## Types and expressions

- A block evaluates to its last expression, and `;` turns an expression into a
  statement producing `()`. That's why a function ends without `return`, and why
  `match` and `if` can produce values.
- `{}` is `Display` (for users), `{:?}` is `Debug` (for developers).
- `_` in a type means "infer this", and works when only one answer is possible.
  Function signatures and struct fields must be written out in full.

## Habits

- Read compiler errors top to bottom: labels show *why*, `help:` shows the fix.
- Treat warnings as bug reports. "Unused variable" usually means I forgot to use
  the thing I meant to use.
- `cargo fmt` and `cargo clippy` before committing.
- `cargo doc -p <crate> --open` documents the exact version in `Cargo.lock`.

## To revisit

- [ ] Book chapter 4: the borrowing rule (many `&` readers or one `&mut` writer).
- [ ] Book chapter 6: match on `e.kind()` to tell `InvalidDigit` from `PosOverflow`.
- [ ] Book chapter 10: lifetimes, and reading `where` clauses in the docs.
- [ ] Raindrops as an iterator chain (`filter`, `map`, `collect`) with no `mut`.
- [ ] `Cow<str>`, to return borrowed or owned text without always allocating.
