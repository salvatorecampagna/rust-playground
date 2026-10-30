# Variable

The Rust Programming Language,
[chapter 3.1](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing).

```bash
cargo run -p variable
```

## Notes

- **Shadowing can change the type.** `x` starts as a `&str` and becomes a
  `usize` (its length). With `let mut` that's a type error, because mutation
  keeps the type.
- **This isn't about immutability.** Each `let` is a brand-new variable that
  hides the old one. What it saves is inventing names like `x_str` and `x_len`.
- **Java has no equivalent.** There you'd always need a second name.
