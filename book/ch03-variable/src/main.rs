//! Shadowing with a type change: each `let` declares a new variable that
//! hides the previous one, even when it reuses the name.
fn main() {
    let x = "this is a test";
    println!("The value of x is {:?}", x);

    // NOTE: the new `x` is a `usize` (the length); the old `&str` is hidden.
    let x = x.len();
    println!("The value of x is {}", x);
}
