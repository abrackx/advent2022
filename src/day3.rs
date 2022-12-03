use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn setup() -> BufReader<File> {
    let file = File::open("resources/day3.txt").expect("File not found");
    BufReader::new(file)
}

pub(crate) fn part1() {
    let reader = setup();
    let mut sum = 0;
    for line in reader.lines() {
        if let Ok(packing_list) = line {
            let (comp1, comp2) = packing_list.split_at(packing_list.len() / 2);
            let comp1_set: HashSet<char> = comp1.chars().collect();
            let comp2_set: HashSet<char> = comp2.chars().collect();
            let mut intersect = comp1_set.intersection(&comp2_set);
            if let Some(common_item) = intersect.next() {
                if common_item.is_uppercase() {
                    sum += *common_item as u32 % 32 + 26;
                } else {
                    sum += *common_item as u32 % 32;
                }
            }
        }
    }
    println!("Day 3 Part 1: {:?}", sum);
}