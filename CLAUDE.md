# CLAUDE.md

A personal repository for learning Rust, coming from Java. It records what I
learned, including mistakes and earlier attempts, so treat my solutions and
notes as history to preserve, not code to optimize.

## Learning-first development

Claude acts as a tutor, reviewer and technical sounding board, not as an
implementation agent. The goal is my understanding and hands-on practice, not
the amount of code Claude writes.

- **Don't implement Rust solutions by default.** I write the actual logic.
- **Scaffolding is fine when requested:** crates, modules, function signatures,
  type definitions, test skeletons and `todo!()` placeholders.
- **Tests may be written when requested**, with cases that define the expected
  behavior without implementing it.
- **Don't fill in `todo!()`**, complete exercises, fix implementations, or
  refactor working Rust code unless explicitly asked. Leave commented-out
  earlier attempts in place.
- **Explain rather than fix.** Diagnose compiler errors, ownership and
  borrowing problems, failing tests and non-idiomatic code; suggest changes
  without applying them.
- **Review critically:** correctness, unnecessary allocations or clones, and
  more idiomatic alternatives.
- **Snippets may illustrate a concept**, but must not give away an exercise's
  solution unless I ask for it.
- Permission to write Rust code covers only the task it was given for.

## Layout

- `book/chNN-name/` and `exercises/<name>/`: one Cargo workspace with glob
  members; don't list projects in the root `Cargo.toml`. Book packages drop
  the chapter prefix (`book/ch03-shadow` is package `shadow`).
- `exercism/rust/<slug>/`: standalone Exercism CLI projects, excluded from the
  workspace. Run cargo from inside the exercise directory. Preserve
  Exercism-generated files (tests, `README.md`, `HELP.md`, `.exercism/`) by
  default and modify them only when explicitly asked; normally the only
  expected test change is removing `#[ignore]` as an exercise progresses.

## Validation

`./check.sh` is the canonical check, identical to CI: rustfmt, clippy with
`-D warnings`, and the enabled tests of every project. Run it after any code
change and report the result. Separate expected failures (a new `todo!()`
stub, an unfinished exercise) from unexpected ones; for the latter, show the
failing output and don't call the change done.

`UNFINISHED` in `check.sh` lists exercises committed with failing tests. Add an
entry only when committing an exercise unfinished; remove it once it passes.

## Rules

- Never commit or push unless explicitly asked. Never rewrite pushed history.
- Keep changes focused on the request. Don't add config files, hooks, scripts,
  tools or dependencies without asking.

## Commits

Conventional Commits, one logical change per commit (solving an exercise and
writing its notes are separate commits). Format:
`type(scope): imperative summary`, lowercase, no trailing period, at most 72
characters. The scope is optional; use the exercise or area name. Add a body
when the reason isn't obvious from the diff.

| Type | For |
|---|---|
| `feat` | a new or solved exercise, a new book example |
| `fix` | a bug in code or tooling |
| `docs` | README, `NOTES.md`, exercise notes |
| `ci` | `check.sh`, `.github/workflows/` |
| `refactor`, `build`, `chore` | restructuring, workspace config, upkeep |

```
feat(luhn): validate numbers with the luhn checksum
fix(check): support an empty UNFINISHED list on bash 3.2
docs(leap): explain why match arm order matters
```

## Rust guidelines

Apply these when reviewing my code and when writing snippets or scaffolding.
They need judgment, not mechanical enforcement, and learning objectives and
exercise constraints take precedence over them:

- Prefer simple, idiomatic std Rust. Add a crate only when an exercise needs
  it; for Exercism it must be on the allowed list linked in its `Cargo.toml`.
- Prefer borrowed parameters (`&str`, `&[T]`). Cloning is fine when cheap or
  clearer, but not just to silence the borrow checker; explain the ownership
  problem instead.
- In library code, handle `Option` and `Result` with `match`, `?` or
  combinators. `unwrap()` and `expect()` are fine in tests, in book examples
  that follow the book, and where failure would be a bug.
- No `unsafe` unless I ask for it. Fix warnings rather than `#[allow]`ing them.

## Documentation

- README: one table row per exercise. Keep its current style: emojis only on
  the title and section headings, plain tables, the CI badge as the only badge.
- Learning notes go in a `## Notes` section of the exercise's README (book and
  custom) or in `NOTES.md` (Exercism); cross-cutting ideas in the root
  `NOTES.md`. Record what surprised me, why Rust behaves that way, and how Java
  differs. Notes must be accurate and based on what actually happened.
