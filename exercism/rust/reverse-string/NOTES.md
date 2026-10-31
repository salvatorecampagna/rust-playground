# Notes: Reverse String

[Exercise](https://exercism.org/tracks/rust/exercises/reverse-string) ·
[Solution](src/lib.rs)

## Learnings

### #1 — Reversing `.chars()` breaks some text

`.chars()` yields Unicode code points, but `ü` can be two: `u` plus a
combining diaeresis (`U+0308`). Reversing moves the dots onto the previous
letter, and Thai vowel marks break the same way. That's what the `grapheme`
bonus tests check.

**References:** [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars) ·
[`char`](https://doc.rust-lang.org/std/primitive.char.html)

### #2 — A string has three levels of "character"

For a decomposed `ü`:

| Level | API | Count |
|---|---|---|
| UTF-8 bytes | `.bytes()` | 3 |
| code points | `.chars()` | 2 |
| grapheme clusters | `unicode-segmentation` | 1 |

The standard library leaves graphemes to a crate because the Unicode rules
are complex and keep changing.

**References:** [Rust Book — Bytes and Scalar Values and Grapheme Clusters! Oh My!](https://doc.rust-lang.org/book/ch08-02-strings.html#bytes-and-scalar-values-and-grapheme-clusters-oh-my) ·
[`UnicodeSegmentation::graphemes`](https://docs.rs/unicode-segmentation/1.13.3/unicode_segmentation/trait.UnicodeSegmentation.html#tymethod.graphemes)

### #3 — `.graphemes()` needs the trait import

`.graphemes()` only works with `use unicode_segmentation::UnicodeSegmentation;`.
The crate implements its trait for `str`, and the import brings the trait's
methods into scope; remove it and the method call stops compiling. A crate
can add methods to an existing type this way, which Java can't do for
`String`.

**References:** [Rust Reference — Method-call expressions](https://doc.rust-lang.org/reference/expressions/method-call-expr.html) ·
[Rust Book — Implementing a Trait on a Type](https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type)

### #4 — `collect()` knows to build a `String`

It knows because the function returns `String`. The turbofish isn't needed.

**References:** [`Iterator::collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)

### #5 — Java's `char` is a level lower

In Java, `char` is a UTF-16 code unit, a level below Rust's `char`.
`StringBuilder.reverse()` keeps surrogate pairs together but still breaks
combining marks.

**References:** [`char`](https://doc.rust-lang.org/std/primitive.char.html)
(Rust's side of the comparison).

Run the bonus tests with `cargo test --features grapheme`.
