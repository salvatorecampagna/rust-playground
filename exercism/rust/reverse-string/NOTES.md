# Notes: Reverse String

[Exercise](https://exercism.org/tracks/rust/exercises/reverse-string) ·
[Solution](src/lib.rs)

- **Reversing `.chars()` breaks some text.** `.chars()` yields Unicode code
  points, but `ü` can be two: `u` plus a combining diaeresis (`U+0308`).
  Reversing moves the dots onto the previous letter, and Thai vowel marks
  break the same way.
- **A string has three levels of "character".** For a decomposed `ü`:

  | Level | API | Count |
  |---|---|---|
  | UTF-8 bytes | `.bytes()` | 3 |
  | code points | `.chars()` | 2 |
  | grapheme clusters | `unicode-segmentation` | 1 |

  The standard library leaves graphemes to a crate because the Unicode rules
  are complex and keep changing.
- **`use UnicodeSegmentation;` is what adds `.graphemes()` to `&str`.** Remove
  the import and the method disappears. Importing a trait adds methods to
  existing types, which Java interfaces can't do for `String`.
- **`collect()` knows to build a `String`** because the function returns
  `String`. The turbofish isn't needed.
- **In Java**, `char` is a UTF-16 code unit, a level below Rust's `char`.
  `StringBuilder.reverse()` keeps surrogate pairs together but still breaks
  combining marks.

Run the bonus tests with `cargo test --features grapheme`.
