# Notes: Raindrops

[Exercise](https://exercism.org/tracks/rust/exercises/raindrops) ·
[Solution](src/lib.rs)

- **The `match` from FizzBuzz doesn't scale.** Three rules took 8 arms, and
  every new rule doubles that (2ⁿ). The rules are independent and each one adds
  its sound, so this is a loop problem, not a "one winner" problem like
  [Leap](../leap/NOTES.md).
- **Progression:** an 8-arm `match`, then the sounds moved into an array (but
  the `match` still hard-coded each index), then a loop over the rules. The
  rules are now data passed to `sounds(n, rules: &[(u32, &str)])`, so adding a
  sound is one line.
- **`&[T]` instead of `[T; 3]`.** An array's length is part of its type, and a
  slice accepts any length. The caller passes `&[...]`, which lends the rules
  without giving them away.
- **Looping over a slice yields references.** `divisor` was a `&u32`, and
  `is_multiple_of` wants a `u32`. The fix is `*divisor`, or the pattern
  `&(divisor, sound)`. The pattern works because both fields are `Copy`.
- **Bug I hit:** the "nothing matched, return the number" check was inside the
  loop, giving `"10Plang"`. "Nothing matched" is only known after the loop.
- **In Java** I'd reach for `StringBuilder`. In Rust, `String` already is the
  growable buffer; `let mut` is what allows appending.
