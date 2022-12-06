use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}


fn main() {
    // println!("Hello, world!");
    let res = read_file("input");
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
