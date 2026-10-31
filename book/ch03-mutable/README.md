# Mutable

The Rust Programming Language,
[chapter 3.1](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html).

```bash
cargo run -p mutable
```

## Learnings

### #1 — Variables are immutable unless declared `let mut`

Assigning to a plain `let` binding is a compile error (E0384). Java has the
opposite default: everything is mutable unless marked `final`.

**References:** [Rust Book — Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability) ·
[E0384](https://doc.rust-lang.org/error_codes/E0384.html)

### #2 — `mut` keeps the type

`mut` changes the value in place, so the type stays the same. Changing the
type needs shadowing instead (see [`ch03-variable`](../ch03-variable)).

**References:** [Rust Book — Shadowing](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing)
