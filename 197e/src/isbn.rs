fn to_digit(character: char) -> Option<u32> {
    match character {
        'x' | 'X' => Some(10),
        _ => char::to_digit(character, 10),
    }
}

fn split_isbn(isbn: &'_ str) -> Vec<u32> {
    let mut digits: Vec<_> = isbn.chars().filter_map(to_digit).collect();

    if digits.len() > 10 {
        digits.truncate(10);
        return digits;
    }

    // This matters because the isbn check digit is calculated
    // based on where the digits are placed. Inserting at the front
    // preserves the expected value
    while digits.len() < 10 {
        digits.insert(0, 0);
    }

    digits
}

pub fn is_valid(isbn: &'_ str) -> bool {
    let digits = split_isbn(isbn);
    let multipliers = (1..=10).rev();

    let sum: u32 = digits.iter()
                    .zip(multipliers)
                    .map(|(digit, multiplier)| digit * multiplier)
                    .sum();

    sum % 11 == 0
}