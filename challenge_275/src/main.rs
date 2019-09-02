fn main() {
    let tests = vec![("Spenglerium", "Ee"), 
                     ("Zeddemorium", "Zr"), 
                     ("Venkmine", "Kn"), 
                     ("Stantzon", "Zt"), 
                     ("Melintzum", "Nn"),
                     ("Tullium", "Ty"),
                    ];
    
    println!("Element symbol validation:");
    tests.iter()
        .map(|(name, symbol)| (name, symbol, is_valid(name, symbol)))
        .for_each(|result| println!("{} \t=> {}? {}", result.0, result.1, result.2));

    println!("\nPossible symbols for each element:");
    tests.iter()
        .map(|(name, _)| (name, get_distinct_symbols(name)))
        .for_each(|result| println!("Element \"{}\" has {} valid symbols", result.0, result.1.len()));
}

//Steps:
//Convert the inputs to lowercase so case isn't part of the question
//Retrieve the first and second char from the symbol (Ee)
//Match against the Options from name.find and name.rfind
//find searches from the left, and rfind searches from the right
//This can be exploited to determine if chars appear in left to right order,
//and if a symbol with duplicate letters shows up correctly per the rules
fn is_valid(name: &str, symbol: &str) -> bool {
    let name = name.to_lowercase();
    let symbol = symbol.to_lowercase();

    let first = symbol.chars().nth(0).unwrap();
    let second = symbol.chars().nth(1).unwrap();
    
    match (name.find(first), name.rfind(second)) {
        (None, _) | (_, None) => false,
        (Some(pos1), Some(pos2)) => pos1 < pos2,
    }
}

//Challenge 2 of this puzzle is solved by utilizing the fact that hashset
//does not permit duplicate entries. Simply iterate over the name left to
//right with two loops, one indexing the first char and the other indexing
//every char in the name after the first chat. Format the first to uppercase,
//and then insert into the hashset.
use std::collections::HashSet;
fn get_distinct_symbols(name: &str) -> HashSet<(char, char)> {
    let mut symbols: HashSet<(char, char)> = HashSet::new();
    
    for first in 0..name.len() - 1 {
        for second in first + 1..name.len() {
            let ch1 = name.to_uppercase().chars().nth(first).unwrap();
            let ch2 = name.to_lowercase().chars().nth(second).unwrap();
            
            symbols.insert((ch1, ch2));
        }
    }

    symbols
}