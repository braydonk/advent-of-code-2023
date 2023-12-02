use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod part_one;
mod part_two;

fn main() {
    part_one::run_puzzle("./puzzle1.txt");
    part_two::run_puzzle("./puzzle1.txt");
}


// Copied shamelessly from Rust By Example.
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}