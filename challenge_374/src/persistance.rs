/// Calculate additive persistance for a number, return it as `Some(persistance)`
///
/// If the input is less than 10, return `None` because the persistance is solved
fn collapse(mut number: u128) -> Option<u128> {
    if number < 10 {
        return None
    }

    let mut sum = 0;
    while number > 0 {
        sum += number % 10;
        number /= 10;
    }
    
    Some(sum)
}

/// Calculates the additive persistance for a number
pub fn persistance(mut number: u128) -> usize {
    let mut iterations = 0;
    while let Some(x) = collapse(number) {  
        number = x;
        iterations += 1;
    }

    iterations
}