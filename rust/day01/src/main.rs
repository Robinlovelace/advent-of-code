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
    println!("{}", res);
}
