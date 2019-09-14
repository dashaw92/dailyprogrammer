mod upc;

fn main() {
    let input = std::env::args().nth(1).expect("arg1 should be the UPC to calculate the check digit for");
    println!("{}", upc::calc_check_digit(&input));
}

#[cfg(test)]
mod tests {
    use super::upc::calc_check_digit;

    #[test]
    fn test_upc() {
        assert_eq!(calc_check_digit("4210000526"), 4);
        assert_eq!(calc_check_digit("3600029145"), 2);
        assert_eq!(calc_check_digit("12345678910"), 4);
        assert_eq!(calc_check_digit("1234567"), 0);
    }
}