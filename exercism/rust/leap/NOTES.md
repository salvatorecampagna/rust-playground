# Notes: Leap

[Exercise](https://exercism.org/tracks/rust/exercises/leap) ·
[Solution](src/lib.rs)

- **First solution:** one boolean expression with `is_multiple_of`. The current
  one matches on `(year % 4, year % 100, year % 400)` for the pattern-matching
  practice.
- **A pattern can't say "non-zero".** `_` matches anything, including 0. The
  "not divisible" part comes from arm order: arms are checked top to bottom
  and the first match wins, so a later `_` only sees values the earlier arms
  didn't take.
- **The rules are a chain of exceptions, written bottom-up:** ÷400 is leap,
  then ÷100 is not leap, then ÷4 is leap, then everything else is not. My
  wrong attempts had every arm return `true`, or left out the ÷100 arm so the
  ÷4 arm swallowed 1900.
- **A Karnaugh map helped.** With A = ÷4, B = ÷100, C = ÷400, most cells are
  impossible (C → B → A), and those don't-cares are why so many positions
  can be `_`.
- **In Java** this would usually be an `if`/`else if` chain. Java has no
  tuples, so matching three values in one `switch` needs a record and record
  patterns (Java 21).
