pub fn raindrops(n: u32) -> String {
    sounds(
        n,
        &[(3, "Pling"), (5, "Plang"), (7, "Plong"), (11, "Plung")],
    )
}

fn sounds(n: u32, rules: &[(u32, &str)]) -> String {
    let mut result = String::new();

    for &(divisor, sound) in rules {
        if n.is_multiple_of(divisor) {
            result.push_str(sound);
        }
    }

    if result.is_empty() {
        return n.to_string();
    }

    result
}
