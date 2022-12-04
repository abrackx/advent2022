mod day1;
mod day2;
mod day3;
mod day4;

extern crate core;

use std::fs::File;
use std::io::BufReader;

fn main() {
    day1::part1();
    day1::part2();
    day2::part1();
    day2::part2();
    day3::part1();
    day3::part2();
    day4::part1();
    day4::part2();
}

fn setup(file: &str) -> BufReader<File> {
    let file = File::open(file).expect("File not found");
    BufReader::new(file)
}


