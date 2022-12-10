use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    // Create Rock, Paper, Scissors lookup table:
    // The score for a single round is the score for the shape you selected
    // (1 for Rock, 2 for Paper, and 3 for Scissors):
    let mut lookup_value = HashMap::new();
    lookup_value.insert("Rock".to_string(), 1);
    lookup_value.insert("Paper".to_string(), 2);
    lookup_value.insert("Scissors".to_string(), 3);
    // Create a lookup table from letter A to C to Rock, Paper, Scissors:
    let mut lookup_item = HashMap::new();
    lookup_item.insert("A".to_string(), "Rock".to_string());
    lookup_item.insert("B".to_string(), "Paper".to_string());
    lookup_item.insert("C".to_string(), "Scissors".to_string());
    // Create a lookup table from X to Z to Rock, Paper, Scissors:
    lookup_item.insert("X".to_string(), "Rock".to_string());
    lookup_item.insert("Y".to_string(), "Paper".to_string());
    lookup_item.insert("Z".to_string(), "Scissors".to_string());

    // Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock
    // A defeats C, C defeats B, and B defeats A:
    let mut lookup_win = HashMap::new();
    // First player wins, 0 points:
    lookup_win.insert("Rock Scissors".to_string(), 0);
    lookup_win.insert("Scissors Paper".to_string(), 0);
    lookup_win.insert("Paper Rock".to_string(), 0);
    // Second player wins, 6 points:
    lookup_win.insert("Scissors Rock".to_string(), 6);
    lookup_win.insert("Paper Scissors".to_string(), 6);
    lookup_win.insert("Rock Paper".to_string(), 6);
    // Draw, 3 points:
    lookup_win.insert("Rock Rock".to_string(), 3);
    lookup_win.insert("Paper Paper".to_string(), 3);
    lookup_win.insert("Scissors Scissors".to_string(), 3);
    
    // Print the lookup table:
    println!("{:?}", lookup_item);
    // Read the file
    let contents = read_to_string("input.test").unwrap();
    // Take the first line:
    let first_line = contents.lines().next().unwrap();
    // Split the line into words:
    let words: Vec<String> = first_line.split_whitespace().map(|word| word.to_string()).collect();
    // // Lookup items:
    let item1 = &lookup_item[&words[0]];
    let items: Vec<String> = words.iter().map(|word| lookup_item[word].to_string()).collect();
    // // Lookup values:
    let values: Vec<i32> = items.iter().map(|item| lookup_value[item]).collect();
    // // Print the values:
    println!("{:?}", items);
    println!("{:?}", values);
}
