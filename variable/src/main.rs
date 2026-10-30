//! Demonstrates shadowing: each `let` declares a brand-new variable,
//! even when it reuses an existing name. The new variable hides the
//! old one and may have a different type (here `&str`, then `usize`),
//! so there's no need to invent names like `x_str` and `x_len`.
//!
//! Unlike `mut`, which changes a value in place and must keep its type,
//! shadowing creates a new variable and leaves the old value untouched.
fn main() {
    let x = "this is a test";
    println!("The value of x is {:?}", x);

    let x = x.len();
    println!("The value of x is {}", x);
}
