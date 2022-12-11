use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    // Total value:
    let mut total = 0;
    // All letters from a to z:
    let characters_lower_case: Vec<char> = ('a'..='z').collect();
    // All letters from A to Z:
    let characters_upper_case: Vec<char> = ('A'..='Z').collect();
    // Combine the two vectors:
    let characters: Vec<char> = characters_lower_case
        .iter()
        .chain(characters_upper_case.iter())
        .cloned()
        .collect();
    // Print the characters:
    // println!("{:?}", characters);
    // Hashmap:
    let mut char_vals: HashMap<char, usize> = HashMap::new();
    // For each character:
    for (c, i) in characters.iter().enumerate() {
        char_vals.insert(*i, c + 1);
        // println!("{} {}", i, c + 1);
    }
    // Read in file:
    let contents = read_to_string("input.txt").unwrap();
    // Print the contents:
    // println!("{}", contents);
    // // get a single line:
    // let line = contents.lines().next().unwrap();
    // For each line:
    for line in contents.lines() {
        // Print the line:
        // println!("{}", line);
        // Number of characters in the line:
        let len = line.chars().count();
        // Half of the length:
        let half = len / 2;
        // Split the line into two halves:
        let (first, second) = line.split_at(half);
        // Check for shared characters in the two halves:
        for (_i, c) in first.chars().enumerate() {
            // For loop over characters in second half:
            for (_j, d) in second.chars().enumerate() {
                // Check if the characters are the same:
                if c == d {
                    // Print the characters:
                    // println!("{} {}", c, d);
                    // Print the index of the characters:
                    // println!("{} {}", i, j);
                    // Print the value of the first matching character:
                    // println!("{}", char_vals.get(&c).unwrap());
                    // Add the value to the total:
                    total += char_vals.get(&c).unwrap();
                }
            }
        }
    }
    // Print the total:
    println!("Total: {}", total);
}
