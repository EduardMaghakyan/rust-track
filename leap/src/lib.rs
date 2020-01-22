pub fn is_leap_year(year: u64) -> bool {
    if year % 4 == 0 {
        return if year % 100 == 0 { year % 400 == 0 } else { true }
    }
    false
}

