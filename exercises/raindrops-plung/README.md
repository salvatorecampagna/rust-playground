# Raindrops: Plung

A follow-up to Exercism's Raindrops exercise, with a fourth sound.

## Instructions

Your task is to convert a number into its corresponding raindrop sounds.

If a given number:

- is divisible by 3, add "Pling" to the result.
- is divisible by 5, add "Plang" to the result.
- is divisible by 7, add "Plong" to the result.
- is divisible by 11, add "Plung" to the result.
- **is not** divisible by 3, 5, 7, or 11, the result should be the number as a string.

Sounds always appear in that order: Pling, Plang, Plong, Plung.

## Examples

- 22 is divisible by 11, but not 3, 5, or 7, so the result is `"Plung"`.
- 33 is divisible by 3 and 11, so the result is `"PlingPlung"`.
- 1155 is divisible by 3, 5, 7, and 11, so the result is `"PlingPlangPlongPlung"`.
- 13 is not divisible by 3, 5, 7, or 11, so the result is `"13"`.

## The real challenge

Solving this with one `match` arm per combination takes 2^4 = 16 arms.
Try to write a solution where adding a fifth sound would mean adding only
one line.

## Running the tests

The exercise starts with every test but the first marked `#[ignore]`; remove
the next `#[ignore]` in `tests/raindrops_plung.rs` each time the current test
passes. This solution is complete, so all tests are enabled:

```bash
cargo test -p raindrops-plung
```

## Notes

- **The fourth rule was a one-line change.** I copied `sounds()` from my
  [Raindrops solution](../../exercism/rust/raindrops/src/lib.rs) and only added
  `(11, "Plung")` to the rules array; all 22 tests passed. The 8-arm `match`
  version would have needed 16 arms.
- **The design follows the shape of the problem.** The rules are independent,
  so they work as data in a loop. `match` fits when exactly one case wins, as
  in [Leap](../../exercism/rust/leap/NOTES.md).
