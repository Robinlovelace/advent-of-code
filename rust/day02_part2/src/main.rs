use std::fs::read_to_string;

// Create enum for the items:
// Traits: Clone (for copying), Copy (for copying), PartialEq (for comparing)
// https://doc.rust-lang.org/std/marker/trait.Copy.html
// https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
#[derive(Clone, Copy, PartialEq)]
enum Item {
    Rock,
    Paper,
    Scissors,
}

// Use enum to avoid :: syntax:
use Item::*;

// Implementations:
impl Item {
    // Function to calculate the number of points for each item:
    fn points(&self) -> usize {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
    // Function to calculate points from winning:
    fn outcome(&self, other: &Item) -> usize {
        // 3 points for a draw:
        if self == other {
            return 3;
        }
        // 6 points for a win:
        // Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock
        match (self, other) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 6,
            _ => 0,
        }
    }
    // Parse A to C and X to Z to Item:
    fn parse(s: &str) -> Item {
        match s {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => panic!("Invalid item: {}", s),
        }
    }
    // Parse
    fn parse2(s: &str) -> Item {
        match s {
            // Opposition is Rock:
            "A X" => Scissors,
            "A Y" => Rock,
            "A Z" => Paper,
            // Opposition is Paper:
            "B X" => Rock,
            "B Y" => Paper,
            "B Z" => Scissors,
            // Opposition is Scissors:
            "C X" => Paper,
            "C Y" => Scissors,
            "C Z" => Rock,
            _ => panic!("Invalid item: {}", s),
        }
    }
    // // Function to convert Item to String:
    // fn toString(&self) -> String {
    //     match self {
    //         Rock => "Rock".to_string(),
    //         Paper => "Paper".to_string(),
    //         Scissors => "Scissors".to_string(),
    //     }
    // }
}

// Implementation to calculate value:

fn main() {
    // Create a variable to store the total points:
    let mut total = 0;
    let contents = read_to_string("input.txt").unwrap();
    // Iterate over the lines:
    for line in contents.lines() {
        // Split the line into words:
        let words: Vec<String> = line
            .split_whitespace()
            .map(|word| word.to_string())
            .collect();
        // Opponent's item:
        let item1 = Item::parse(&words[0]);
        // Player's item:
        let item2 = Item::parse2(&line);
        // println!("Opponent's item: {}", item1.toString());
        // println!("Player's item: {}", item2.toString());
        // Calculate the value of player's item:
        let value2 = item2.points();
        // Calculate the value of the outcome:
        let winner = item2.outcome(&item1);
        // Print the winner:
        // println!("The winner is {} with {} points and a win value of {}", item2, value2, winner);
        total += winner + value2;
    }
    println!("Total points: {}", total);
}
