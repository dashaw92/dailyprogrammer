mod isbn;

use std::env;
use isbn::is_valid;

fn main() {
    let isbn = env::args().nth(1).expect("Missing ISBN as argument.");
    println!("{}", is_valid(&isbn));
}
