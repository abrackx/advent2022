extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../resources/day1-p1.txt").expect("File not found");
    let reader = BufReader::new(file);
    let mut temp_sum = 0;
    let mut max_cal = 0;
    for line in reader.lines() {
        if line.as_ref().unwrap().is_empty() {
            temp_sum = 0;
            continue;
        }
        temp_sum += line.unwrap().parse::<i32>().unwrap();
        if temp_sum > max_cal {
            max_cal = temp_sum;
        }
    }
    println!("{:?}", max_cal);
}
