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

pub(crate) fn part2() {
    let reader = setup();
    let mut lines = reader.lines();
    let mut sum = 0;
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        if let (Ok(list1), Ok(list2), Ok(list3)) = (line1, line2, line3) {
            let list1_set: HashSet<char> = list1.chars().collect();
            let list2_set: HashSet<char> = list2.chars().collect();
            let list3_set: HashSet<char> = list3.chars().collect();
            let sets = &vec![list1_set, list2_set, list3_set];
            let intersect = sets[0].iter().filter (move |c| sets[1..].iter().all (|s| s.contains (c))).collect::<Vec<_>>();
            if let Some(common_item) = intersect.get(0) {
                if common_item.is_uppercase() {
                    sum += **common_item as u32 % 32 + 26;
                } else {
                    sum += **common_item as u32 % 32;
                }
            }
        }
    }
    println!("Day 3 Part 2: {:?}", sum);
}