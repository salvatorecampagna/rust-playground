use std::cmp::Ordering;
use std::io;

use rand::RngExt;

fn main() {
    let valid_range = 1..=100;
    println!(
        "Guessing game. Guess a number between {} and {}",
        valid_range.start(),
        valid_range.end()
    );

    let secret_number = rand::rng().random_range(valid_range.clone());
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Retry.");
                continue;
            }
        };
        if !valid_range.contains(&guess) {
            println!("{guess} is outside the {valid_range:?} range.");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("{guess} is too small.");
            }
            Ordering::Greater => {
                println!("{guess} is too large.");
            }
            Ordering::Equal => {
                println!("{guess} matches {secret_number}! You win!");
                break;
            }
        }
    }
}
