pub fn is_armstrong_number(num: u32) -> bool {
    let exp = match num.checked_ilog10() {
        None => 1,
        Some(log) => log + 1,
    };
    let mut n = num;
    let mut sum: u32 = 0;
    while n > 0 {
        let digit = n % 10;
        n /= 10;
        sum = match sum.checked_add(digit.pow(exp)) {
            None => return false,
            Some(x) => x,
        }
    }

    sum == num
}

#[cfg(test)]
mod tests {
    use super::is_armstrong_number;

    #[test]
    fn test_is_armstrong_overflow_returns_false() {
        assert!(!is_armstrong_number(3_999_999_999));
    }
}
