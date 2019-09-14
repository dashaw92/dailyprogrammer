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

    //pad the vec to be 11 digits long. A upc is 12 digits
    // while digits.len() < 11 {
    //     digits.insert(0, 0);
    // }

    //I realized that the padding above is kinda pointless?
    //It doesn't matter if the vec is 11 digits or n, as long
    //as it's odd. Padding it to 11 long with 0s is pointless:
    //0 + n = n
    if digits.len() < 11 && digits.len() & 1 != 1 {
        digits.insert(0, 0);
    }

    digits
}

fn sum_odd(digits: &[u32]) -> u32 {
    digits.iter().step_by(2).map(|x| x * 3).sum()
}

fn sum_even(digits: &[u32]) -> u32 {
    digits.iter().skip(1).step_by(2).sum()
}

pub fn calc_check_digit(input: &'_ str) -> u32 {
    let digits = to_upc(input);
    let total = sum_odd(&digits) + sum_even(&digits);
    let remainder = total % 10;

    if remainder == 0 {
        remainder
    } else {
        10 - remainder
    }
}