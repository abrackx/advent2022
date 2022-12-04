use std::io::BufRead;
use crate::setup;

pub(crate) fn part1() {
    let reader = setup("resources/day4.txt");
    let mut sum = 0;
    let _ = reader.lines()
        .filter_map(|line| line.ok())
        .map(|line| line.split(",").map(String::from).collect::<Vec<String>>())
        .for_each(|pair| {
            let (group1, group2) = (pair[0].as_str(), pair[1].as_str());
            let (start1_str, end1_str) = group1.split_once("-").unwrap();
            let (start2_str, end2_str) = group2.split_once("-").unwrap();
            let (start1, start2) = (start1_str.parse::<i32>().unwrap(), start2_str.parse::<i32>().unwrap());
            let (end1, end2) = (end1_str.parse::<i32>().unwrap(), end2_str.parse::<i32>().unwrap());
            if (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1) {
                sum += 1;
            }
        });
    println!("Day 4 Part 1: {:?}", sum);
}

pub(crate) fn part2() {
    let reader = setup("resources/day4.txt");
    let mut sum = 0;
    let _ = reader.lines()
        .filter_map(|line| line.ok())
        .map(|line| line.split(",").map(String::from).collect::<Vec<String>>())
        .for_each(|pair| {
            let (group1, group2) = (pair[0].as_str(), pair[1].as_str());
            let (start1_str, end1_str) = group1.split_once("-").unwrap();
            let (start2_str, end2_str) = group2.split_once("-").unwrap();
            let (start1, start2) = (start1_str.parse::<i32>().unwrap(), start2_str.parse::<i32>().unwrap());
            let (end1, end2) = (end1_str.parse::<i32>().unwrap(), end2_str.parse::<i32>().unwrap());
            if (start1 <= start2 && end1 >= start2) || (start2 <= start1 && end2 >= start1) {
                sum += 1;
            }
        });
    println!("Day 4 Part 1: {:?}", sum);
}