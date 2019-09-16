const DECLARATION: &str = include_str!("../input/declaration.txt");
const CIPHER: &str = include_str!("../input/cipher.txt");

fn main() {
    //convert the ciphertext input from "index, index, index, ..." to [usize; index, index, index,...]
    let indexes: Vec<usize> = CIPHER.split(',')
                        .map(|number_str| number_str.trim())
                        .filter_map(|number| number.parse().ok())
                        .collect();
    //split the ciphered texted on whitespace
    let words: Vec<_> = DECLARATION.split_whitespace().collect();

    //for each index in indexes, select the word at the index and get the first
    //char in the word. Collect the resulting chars into a string.
    //the calculated index is the result of bounding it to the length of `words`
    //and subtracting 1. The - 1 is because Beale used 1 based indexing, not 0
    let output: String = indexes.iter()
                            .filter_map(|index| words.get(index % words.len() - 1))
                            .filter_map(|word| word.chars().nth(0))
                            .collect();
                        
    println!("\n{}\n", output);
}