# Notes: Anagram

[Exercise](https://exercism.org/tracks/rust/exercises/anagram) ·
[Solution](src/lib.rs)

## Learnings

### #1 — A string can't be sorted in place

My idea was to lowercase and sort both words, then compare. A `&str` or
`String` can't be sorted: it's UTF-8, and rearranging bytes could produce
invalid text. So I `collect()` the `chars()` into a `Vec<char>` and sort that.
Sorting `char`s rather than bytes is what makes the Greek-letter and
"different characters may have the same bytes" tests pass.

**References:** [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars) ·
[`Iterator::collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) ·
[Rust Book — Bytes and Scalar Values and Grapheme Clusters! Oh My!](https://doc.rust-lang.org/book/ch08-02-strings.html#bytes-and-scalar-values-and-grapheme-clusters-oh-my)

### #2 — `sort()` returns `()`

I chained `.sort()` onto `collect()`, so the sorted `Vec` was thrown away and
both variables held `()`. Then `() == ()` was always true and every candidate
became an anagram: `"listen"` "matched" `google` and `banana`. The `Vec` has to
live in a `let mut` variable, with `.sort()` called on it as its own statement.

**References:** [`slice::sort`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort)

### #3 — `Vec` already has `==`

My `vec_equal` helper checked the lengths and then each element, which is
exactly what `==` on two `Vec`s does. I removed it.

**References:** [`PartialEq` for `Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-PartialEq%3CVec%3CU,+A2%3E%3E-for-Vec%3CT,+A1%3E)

### #4 — Looping over `&[&str]` gives `&&str`

The same lesson as [Raindrops](../raindrops/NOTES.md). Inserting the loop
variable made the set a `HashSet<&&str>`, which didn't match the
`HashSet<&str>` return type (E0308). The pattern `for &possible_anagram`
removes one `&`.

**References:** [Rust Book — Iterating Over the Values in a Vector](https://doc.rust-lang.org/book/ch08-01-vectors.html#iterating-over-the-values-in-a-vector) ·
[Rust Reference — Reference patterns](https://doc.rust-lang.org/reference/patterns.html#reference-patterns)

### #5 — The returned references borrow from an input

I first thought the problem was returning the `HashSet` to the caller. That
part is fine: the set is owned and moves out. The problem was its elements,
references into the caller's strings, with no stated link to any input
(E0621). Putting `'a` on the inner `&str` of `possible_anagrams` says that the
result borrows from those strings. Not from `word`, and not from the slice
itself. The set stores no text of its own; in Java the GC would keep the
strings alive and the question wouldn't come up.

**References:** [Rust Book — Lifetime Annotations in Function Signatures](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-function-signatures) ·
[E0621](https://doc.rust-lang.org/error_codes/E0621.html)

### #6 — A temporary freed while still borrowed

`lowercase_word = word.to_lowercase().trim();` failed with E0716.
`to_lowercase()` builds a new `String` that no variable owns, and `trim()`
returns a `&str` pointing into it. The `String` is freed at the `;`, so the
`&str` would dangle. Storing the `String` in a variable gives it an owner.

**References:** [E0716](https://doc.rust-lang.org/error_codes/E0716.html) ·
[Rust Reference — Temporary scopes](https://doc.rust-lang.org/reference/destructors.html#temporary-scopes)

### #7 — A variable keeps its type

`let mut lowercase_word = word;` made it a `&str`, so assigning
`word.to_lowercase()` later failed with expected `&str`, found `String`
(E0308). The compiler's `help:` suggested removing the method call, which would
have compiled but undone the lowercasing. One `let` with the `String` was the
fix.

**References:** [E0308](https://doc.rust-lang.org/error_codes/E0308.html) ·
[Rust Book — Shadowing](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing)

### #8 — Shadowing hid a value I still needed

I shadowed `possible_anagram` with its lowercase copy, so the insert then
took a `&String` to a local: the wrong spelling (`"carthorse"` instead of
`"Carthorse"`), and a value freed at the end of each iteration. I needed both
versions, so they needed two names.

**References:** [Rust Book — Shadowing](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing) ·
[Rust Book — Dangling References](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references)

### #9 — A helper's name is a promise

My first `sorted_chars` only collected the chars, and the callers still
sorted. Moving the sort inside made the name true and removed the `mut`s and
`.sort()` calls at the call sites. The `Vec` built inside moves out to the
caller as the return value.

**References:** [Rust Book — Return Values and Scope](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#return-values-and-scope)

### #10 — `sort` vs `sort_unstable`

`sort` keeps equal elements in their original order and allocates a buffer;
`sort_unstable` may reorder them, sorts in place and is generally faster.
Stability only matters when equal elements can be told apart, like records
sorted by one field. Two equal `char`s are identical, so `sort_unstable` fits.
Java makes the same split, but chooses for you: `Arrays.sort(int[])` is
unstable, while `Collections.sort` is stable.

**References:** [`slice::sort`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort) ·
[`slice::sort_unstable`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable)
