# Mutable

The Rust Programming Language,
[chapter 3.1](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html).

```bash
cargo run -p mutable
```

## Notes

- **Variables are immutable unless declared `let mut`.** Assigning to a plain
  `let` binding is a compile error (E0384). Java has the opposite default:
  everything is mutable unless marked `final`.
- **`mut` changes the value in place, so the type stays the same.** Changing
  the type needs shadowing instead (see [`ch03-variable`](../ch03-variable)).
