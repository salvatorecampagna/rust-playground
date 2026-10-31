# Notes: Leap

[Exercise](https://exercism.org/tracks/rust/exercises/leap) ·
[Solution](src/lib.rs)

## Learnings

### #1 — I rewrote a working solution as a `match`

My first solution was one boolean expression with `is_multiple_of`. The
current one matches on `(year % 4, year % 100, year % 400)` for the
pattern-matching practice.

**References:** [`u32::is_multiple_of`](https://doc.rust-lang.org/std/primitive.u32.html#method.is_multiple_of) ·
[Rust Reference — Tuple patterns](https://doc.rust-lang.org/reference/patterns.html#tuple-patterns)

### #2 — A pattern can't say "non-zero"

`_` matches anything, including 0. The "not divisible" part comes from arm
order: arms are checked top to bottom and the first match wins, so a later `_`
only sees values the earlier arms didn't take.

**References:** [Rust Reference — `match` expressions](https://doc.rust-lang.org/reference/expressions/match-expr.html) ·
[Rust Reference — Wildcard pattern](https://doc.rust-lang.org/reference/patterns.html#wildcard-pattern)

### #3 — The rules are a chain of exceptions, written bottom-up

÷400 is leap, then ÷100 is not leap, then ÷4 is leap, then everything else is
not. My wrong attempts had every arm return `true`, or left out the ÷100 arm
so the ÷4 arm swallowed 1900.

**References:** [Rust Book — Catch-all Patterns and the `_` Placeholder](https://doc.rust-lang.org/book/ch06-02-match.html#catch-all-patterns-and-the-_-placeholder)

### #4 — A Karnaugh map helped

With A = ÷4, B = ÷100, C = ÷400, most cells are impossible (C → B → A), and
those don't-cares are why so many positions can be `_`.

**References:** No Rust reference; this is a logic-design technique.

### #5 — Java would need a record pattern for this

In Java this would usually be an `if`/`else if` chain. Java has no tuples, so
matching three values in one `switch` needs a record and record patterns
(Java 21).

**References:** No Rust reference; this is a comparison with Java.
