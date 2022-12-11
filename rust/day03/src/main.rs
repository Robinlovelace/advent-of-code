use std::fs::read_to_string;

fn main() {
    // Read in file:
    let contents = read_to_string("input.test").unwrap();
    // Print the contents:
    // println!("{}", contents);
    // get a single line:
    let line = contents.lines().next().unwrap();
    // Print the line:
    println!("{}", line);
    // Number of characters in the line:
    let len = line.chars().count();
    // Half of the length:
    let half = len / 2;
    // Split the line into two halves:
    let (first, second) = line.split_at(half);
    // Check for shared characters in the two halves:
    for(i, c) in first.chars().enumerate() {
        // For loop over characters in second half:
        for (j, d) in second.chars().enumerate() {
            // Check if the characters are the same:
            if c == d {
                // Print the characters:
                println!("{} {}", c, d);
                // Print the index of the characters:
                println!("{} {}", i, j);
            }
        }
    }
}
