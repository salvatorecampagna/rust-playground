fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Error while parsing input");

    if input % 2 == 0 {
        println!("{input} is even");
    } else {
        println!("{input} is odd");
    }

    // NOTE: `if` is an expression: the value of the chosen branch's block
    // becomes the value of the whole `if`, like Java's `cond ? a : b`.
    let x = if input % 2 == 0 { "even" } else { "odd" };
    println!("x: {}", x);

    // NOTE: both branches must have the same type; the compiler reports the
    // mismatch against the first branch ("expected `&str`, found integer").
    // Doesn't compile, E0308: `if` and `else` have incompatible types
    // let y = if input % 2 == 0 { "even" } else { 1 };

    // NOTE: an `if` without `else` has type `()`, because nothing runs when the
    // condition is false, so the `true` branch must produce `()` as well.
    // Doesn't compile, E0317: `if` may be missing an `else` clause
    // let k = if input % 2 == 0 { "odd" };

    // NOTE: an if expression needs a boolean condition
    // Doesn't compile, E0308: mismatched types, expected `bool`, found `i32`
    // if input {
    //     println!("{input} is even");
    // }
}
