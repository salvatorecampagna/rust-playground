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
    //NOTE: next line does not compile: index out of bounds
    //println!("out of bounds: {:?}", weekdays[10]);

    let mut index = String::new();
    println!("Enter an index value");
    std::io::stdin()
        .read_line(&mut index)
        .expect("Error while reading input");
    let index: usize = index.trim().parse().expect("Error while parsing integer");
    println!("index: {}", index);
    println!("day: {:?}", weekdays[index]);

    let x: [u32; 3] = [10, 20, 30];
    println!("x: {:?}", x);

    let y = [100; 7];
    println!("y: {:?}", y);
}
