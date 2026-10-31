# Notes: Raindrops

[Exercise](https://exercism.org/tracks/rust/exercises/raindrops) ·
[Solution](src/lib.rs)

## Learnings

### #1 — The `match` from FizzBuzz doesn't scale

Three rules took 8 arms, and every new rule doubles that (2ⁿ). The rules are
independent and each one adds its sound, so this is a loop problem, not a "one
winner" problem like [Leap](../leap/NOTES.md).

**References:** No specific reference; this is a design observation.

### #2 — From an 8-arm `match` to rules as data

I went from an 8-arm `match`, to the sounds in an array (but the `match` still
hard-coded each index), to a loop over the rules. The rules are now data
passed to `sounds(n, rules: &[(u32, &str)])`, so adding a sound is one line.

**References:** [Rust Book — Other Slices](https://doc.rust-lang.org/book/ch04-03-slices.html#other-slices)

### #3 — `&[T]` instead of `[T; 3]`

An array's length is part of its type, and a slice accepts any length. The
caller passes `&[...]`, which lends the rules without giving them away.

**References:** [Rust Book — The Array Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type) ·
[Rust Book — Other Slices](https://doc.rust-lang.org/book/ch04-03-slices.html#other-slices)

### #4 — Looping over a slice yields references

`divisor` was a `&u32`, and `is_multiple_of` wants a `u32`. The fix is
`*divisor`, or the pattern `&(divisor, sound)`. The pattern works because both
fields are `Copy`.

**References:** [Rust Book — Iterating Over the Values in a Vector](https://doc.rust-lang.org/book/ch08-01-vectors.html#iterating-over-the-values-in-a-vector) ·
[Rust Reference — Reference patterns](https://doc.rust-lang.org/reference/patterns.html#reference-patterns) ·
[Rust Reference — The dereference operator](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-dereference-operator)

### #5 — "Nothing matched" is only known after the loop

I put the "nothing matched, return the number" check inside the loop, which
gave `"10Plang"`. It has to come after the loop, once every rule has been
tried.

**References:** No specific reference; this was a logic bug.

### #6 — `String` is already the `StringBuilder`

In Java I'd reach for `StringBuilder`. In Rust, `String` already is the
growable buffer; `let mut` is what allows appending.

**References:** [Rust Book — Updating a String](https://doc.rust-lang.org/book/ch08-02-strings.html#updating-a-string) ·
[`String`](https://doc.rust-lang.org/std/string/struct.String.html)
