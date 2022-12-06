use std::fs::read_to_string;
extern crate human_format;

fn main() {
    let res: String = read_to_string("input").unwrap();
    // Create each 'elf' as a tuple of (id, calories):
    let mut elves: Vec<(usize, i32)> = Vec::new();
    // Count how many elves there are:
    let mut elf = 0;
    // add the contents of each line:
    let mut sum = 0;
    for line in res.lines() {
        let mut sum_elf: i32 = 0;
        // ignore empty lines:
        if line.len() == 0 {
            elves.push((elf, sum_elf));
            elf += 1;
            sum_elf = 0;
            continue;
        }
        let num: i32 = line.parse().unwrap();
        sum_elf += num;
        sum += sum_elf;
    }
    // Convert sum into float
    let sum = sum as f32;
    let n_elves = elf + 1;
    let sum_formatted = human_format::Formatter::new().with_decimals(0).format(sum.into());
    println!("{} calories carried by", sum_formatted);
    println!("{} elves", n_elves);
}
