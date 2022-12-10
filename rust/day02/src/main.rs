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
    // Note: This these three lookups are not needed for the result:
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

    // Variable for total points:
    let mut total = 0;
    
    // // Print the lookup table:
    // println!("{:?}", lookup_item);
    // Read the file
    let contents = read_to_string("input.txt").unwrap();
    // Iterate over the lines:
    for line in contents.lines() {
        // Split the line into words:
        let words: Vec<String> = line.split_whitespace().map(|word| word.to_string()).collect();
        // Lookup items:
        let item1 = &lookup_item[&words[0]];
        let item2 = &lookup_item[&words[1]];
        // Lookup values:
        // let value1 = lookup_value[item1];
        let value2 = lookup_value[item2];
        // Lookup the winner:
        let winner = lookup_win[&format!("{} {}", item1, item2)];
        // Print the winner:
        // println!("The winner is {} with {} points and a win value of {}", item2, value2, winner);
        total += winner + value2;
    }
    println!("Total points: {}", total);
    
}
