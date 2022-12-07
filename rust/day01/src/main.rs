use std::fs::read_to_string;
extern crate human_format;

fn main() {
    // Create each 'elf' as a tuple of (id, calories):
    let mut elves: Vec<(usize, i32)> = Vec::new();
    // Count how many elves there are:
    let mut elf = 0;
    // Add total calorie counter:
    let mut sum = 0;
    // Calories per elf:
    let mut elf_calories = 0; 
    let mut elf_with_max = 0;
    for line in read_to_string("input").unwrap().lines() {
        // ignore empty lines:
        if line.len() != 0 {
            let num: i32 = line.parse().unwrap();
            // Sum of calories for each elf
            elf_calories += num;
            sum += num;
        } else {
            elves.push((elf, elf_calories));
            elf += 1;
            elf_calories = 0;
        }
    }
    // Convert sum into float
    let sum = sum as f32;
    let n_elves = elf + 1;
    let sum_formatted = human_format::Formatter::new().with_decimals(0).format(sum.into());
    println!("{} calories carried by", sum_formatted);
    println!("{} elves", n_elves);
    let sums = elves.iter().map(|x| x.1).collect::<Vec<i32>>();
    let max = sums.iter().max().unwrap();
    println!("{} calories carried by the elf with the most", max);
    for (i, x) in elves.iter().enumerate() {
        if x.1 == *max {
            elf_with_max = i;
        }
    }
    println!("That was elf # {}!", elf_with_max + 1);
    // Find the top 3 elves:
    let mut top_3 = elves.clone();
    top_3.sort_by(|a, b| b.1.cmp(&a.1));
    println!("The top 3 elves are:");
    let mut sum_top = 0;
    for i in 0..3 {
        println!("Elf #{} with {} calories", top_3[i].0 + 1, top_3[i].1);
        sum_top += top_3[i].1;
    }
    println!("In total the top 3 elves carried {} calories", sum_top);
}

