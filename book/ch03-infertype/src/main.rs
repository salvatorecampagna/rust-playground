use std::num::ParseIntError;
use std::str::ParseBoolError;

fn main() {
    let values = ["42", "wow", "-5", "5000000000"];
    for value in values {
        let value: Result<i32, ParseIntError> = value.parse();
        match value {
            Ok(x) => println!("{}", x),
            Err(e) => println!("Error: {}", e),
        }
    }
}
