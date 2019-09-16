// Given an input string, "123456", converts it to a Vec<u32> whose elements are the parsed chars: [1,2,3,4,5,6]
// Will skip invalid base 10 digits: "123ab456" -> [1,2,3,4,5,6]
// If the input string is > 11, it will be right truncated: "1234567890123" -> [1,2,3,4,5,6,7,8,9,0,1]
fn to_upc(input: &'_ str) -> Vec<u32> {
    let mut digits: Vec<_> = input.chars().filter_map(|c| char::to_digit(c, 10)).collect();

    //ensure the vec is at most 11 digits
    if digits.len() > 11 {
        digits.truncate(11);
        return digits
    }

    // It doesn't matter if the vec is 11 digits or n, as long
    // as it's odd. Padding it to 11 long with 0s is pointless:
    // 0 + n = n
    if digits.len() < 11 && digits.len() & 1 != 1 {
        digits.insert(0, 0);
    }

    digits
}

// Rather than go through two functions to get the sum, we'll
// just zip digits iterator with a repeating 3, 1, 3, 1, ... iterator, 
// and then multiply each digit by the corresponding multiplier.
// Starts with 3 because the first digit is considered an odd numbered digit
// in the UPC.
fn sum(digits: &[u32]) -> u32 {
    let multipliers = [3, 1].iter().cycle();

    digits.iter()
        .zip(multipliers)
        .map(|(digit, factor)| digit * factor)
        .sum()
}

pub fn calc_check_digit(input: &'_ str) -> u32 {
    let digits = to_upc(input);
    let total = sum(&digits);
    let remainder = total % 10;

    if remainder == 0 {
        remainder
    } else {
        10 - remainder
    }
}