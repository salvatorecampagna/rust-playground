fn main() {
    let weekdays = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    println!("weekdays: {:?}", weekdays);
    // NOTE: a constant out-of-bounds index is caught at compile time.
    // Doesn't compile, `unconditional_panic` lint: this operation will panic at runtime
    // println!("out of bounds: {:?}", weekdays[10]);

    let mut index = String::new();
    println!("Enter an index value");
    std::io::stdin()
        .read_line(&mut index)
        .expect("Error while reading input");
    let index: usize = index.trim().parse().expect("Error while parsing integer");
    println!("index: {}", index);
    // NOTE: with an index from input, the check happens at run time: `.get()`
    // returns `None` for a bad index (`unwrap_or` supplies a default), while
    // `weekdays[index]` panics.
    println!("day: {:?}", weekdays.get(index).unwrap_or(&"none"));
    println!("day: {:?}", weekdays[index]);

    let x: [u32; 3] = [10, 20, 30];
    println!("x: {:?}", x);

    let y = [100; 7];
    println!("y: {:?}", y);
}
