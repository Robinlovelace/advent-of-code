use std::fs::read_to_string;

// Clone and Copy mean this is a very small object; don't bother with pointers/references ever --
// just copy it.
// PartialEq lets us see if x == y
#[derive(Clone, Copy, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}
// This lets us say Rock instead of Choice::Rock
use Choice::*;

impl Choice {
    /// Does this choice beat the other one? `None` if it's a draw
    fn beats(self, other: Choice) -> Option<bool> {
        // Draw
        if self == other {
            return None;
        }

        Some(match (self, other) {
            (Paper, Rock) => true,
            (Scissors, Paper) => true,
            (Rock, Scissors) => true,
            // All other cases, self loses to other
            _ => false,
        })
    }

    fn value(self) -> usize {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn must_parse(x: &str) -> Choice {
        match x {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("Invalid input {x}"),
        }
    }
}

fn main() {
    let mut total = 0;

    for line in read_to_string("input.txt").unwrap().lines() {
        let choices: Vec<Choice> = line
            .split_whitespace()
            .map(|word| Choice::must_parse(word))
            .collect();
        assert_eq!(choices.len(), 2);

        let score = match choices[0].beats(choices[1]) {
            // Player 1 wins
            Some(true) => 0,
            // Player 2 wins
            Some(false) => 6,
            // Draw
            None => 3,
        };

        total += score + choices[1].value();
    }

    println!("Total points: {total}");
}
