pub fn is_leap_year(year: u64) -> bool {
    match (year % 4, year % 100, year % 400) {
        (0, _, 0) => true,  // divisible by 400 (e.g. 2000)
        (0, 0, _) => false, // divisible by 100 but not by 400 (e.g. 1900)
        (0, _, _) => true,  // divisible by 4 but not by 100 (e.g. 2016)
        _ => false,         // not divisible by 4 (e.g. 2015)
    }
}
