/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let mut digit_count = 0;
    for ch in code.chars().rev() {
        match ch {
            ' ' => { /* skip */ }
            '0'..='9' => {
                let d = ch.to_digit(10).unwrap();
                let d = if digit_count % 2 == 1 {
                    double_digit(d)
                } else {
                    d
                };

                digit_count += 1;
                sum += d;
            }
            _ => return false,
        }
    }
    sum.is_multiple_of(10) && digit_count >= 2
}

fn double_digit(value: u32) -> u32 {
    let doubled = value * 2;
    if doubled > 9 { doubled - 9 } else { doubled }
}
