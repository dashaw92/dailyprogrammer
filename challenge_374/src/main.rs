mod persistance;

use std::env;
use persistance::persistance;

fn main() {
    let arg: u128 = env::args().nth(1).and_then(|x| x.parse().ok()).expect("Missing positive integer as argument.");
    println!("Persistance for {} is {}", arg, persistance(arg))
}

#[cfg(test)]
mod tests {
    use super::persistance::persistance;

    #[test]
    fn test() {
        assert_eq!(persistance(13), 1);
        assert_eq!(persistance(1234), 2);
        assert_eq!(persistance(9876), 2);
        assert_eq!(persistance(199), 3);
    }
}