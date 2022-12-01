use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn setup() -> BufReader<File> {
    let file = File::open("resources/day1-p1.txt").expect("File not found");
    BufReader::new(file)
}

pub(crate) fn part1() {
    let reader = setup();
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
    println!("Day 1 Part 1: {:?}", max_cal);
}

pub(crate) fn part2() {
    let reader = setup();
    let mut temp_sum = 0;
    let mut max_cal: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(3);
    for line in reader.lines() {
        if line.as_ref().unwrap().is_empty() {
            temp_sum = 0;
            continue;
        }
        temp_sum += line.unwrap().parse::<i32>().unwrap();
        if temp_sum > max_cal.peek().unwrap_or(&Reverse(0)).0 {
            if max_cal.len() == 3 {
                max_cal.pop();
            }
            max_cal.push(Reverse(temp_sum));
        }
    }
    let mut sum = 0;
    max_cal.into_iter().for_each(|cal| sum += cal.0 );
    println!("Day 1 Part 2: {:?}", sum);
}