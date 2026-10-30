# Shadow

The Rust Programming Language,
[chapter 3.1](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing).

```bash
cargo run -p shadow
```

## Notes

- **`let x = x + 1;` declares a new `x`.** It doesn't modify the old one, so `x`
  can stay immutable.
- **Shadowing in an inner block ends at its `}`.** Inside the block `x` is 22;
  after it, the outer `x` (11) is visible again.
- **Java rejects this.** Redeclaring a local variable in the same method,
  even in a nested block, is a compile error there.
