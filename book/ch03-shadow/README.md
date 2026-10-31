# Shadow

The Rust Programming Language,
[chapter 3.1](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing).

```bash
cargo run -p shadow
```

## Learnings

### #1 — `let x = x + 1;` declares a new `x`

It doesn't modify the old one, so `x` can stay immutable.

**References:** [Rust Book — Shadowing](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing)

### #2 — Shadowing in an inner block ends at its `}`

Inside the block `x` is 22; after it, the outer `x` (11) is visible again.

**References:** [Rust Book — Shadowing](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing) ·
[Rust Reference — Block expressions](https://doc.rust-lang.org/reference/expressions/block-expr.html)

### #3 — Java rejects this

Redeclaring a local variable in the same method, even in a nested block, is a
compile error there.

**References:** No Rust reference; this is a comparison with Java.
