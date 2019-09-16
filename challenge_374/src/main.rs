mod persistance;

use std::env;
use persistance::persistance;

fn main() {
    let arg: u128 = env::args().nth(1).and_then(|x| x.parse().ok()).expect("Missing positive integer as argument.");
    println!("Persistance for {} is {}", arg, persistance(arg))
}