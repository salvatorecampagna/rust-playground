fn main() {
    let x = 10;
    println!("The value of x is {}", x);

    let x = x + 1;
    println!("The value of x is {}", x);

    {
        let x = x * 2;
        println!("The value of x is {}", x);
    }

    println!("The value of x is {}", x);
}
