fn main() {
    let tests = vec![("Spenglerium", "Ee"), 
                     ("Zeddemorium", "Zr"), 
                     ("Venkmine", "Kn"), 
                     ("Stantzon", "Zt"), 
                     ("Melintzum", "Nn"),
                     ("Tullium", "Ty"),
                    ];
    
    println!("Symbol validation:");
    tests.iter()
         .map(|(name, symbol)| (name, symbol, is_valid(name, symbol)))
         .for_each(|result| println!("{} => {}? {}", result.0, result.1, result.2));

    println!("Valid symbol generator:");
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

    let first = symbol.chars().nth(0).unwrap_or(' ');
    let second = symbol.chars().nth(1).unwrap_or(' ');
    
    match (name.find(first), name.rfind(second)) {
        (None, _) | (_, None) => false,
        (Some(pos1), Some(pos2)) => pos1 < pos2,
    }
}