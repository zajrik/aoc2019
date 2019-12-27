fn digits_never_decrease(password: u32) -> bool {
    let digits: Vec<char> = password.to_string().chars().collect();
    let mut prev_digit: Option<char> = None;

    for digit in digits {
        if prev_digit.is_some() && digit < prev_digit.unwrap() {
            return false;
        }

        prev_digit = Some(digit);
    }

    true
}

fn matching_digits(password: u32) -> bool {
    let digits: Vec<char> = password.to_string().chars().collect();
    let mut prev_digit: Option<char> = None;

    for digit in digits {
        if prev_digit.is_some() && digit == prev_digit.unwrap() {
            return true;
        }

        prev_digit = Some(digit);
    }

    false
}

pub fn part1() -> Result<usize, std::io::Error> {
    let lower_bound: u32 = 183564;
    let upper_bound: u32 = 657474;

    let valid_passwords: usize = (lower_bound..=upper_bound)
        .filter(|p| digits_never_decrease(*p) && matching_digits(*p))
        .count();

    Ok(valid_passwords)
}
