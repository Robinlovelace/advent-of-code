use std::fs::read_to_string;

fn main() {
    // println!("Hello, world!");
    let res: String = read_to_string("input").unwrap();
    // add the contents of each line:
    let mut sum = 0;
    for line in res.lines() {
        // ignore empty lines:
        if line.len() == 0 {
            continue;
        }
        let num: i32 = line.parse().unwrap();
        sum += num;
    }
    println!("{}", sum);
}
