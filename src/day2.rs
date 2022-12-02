use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::day2::Command::{Paper, Rock, Scissors};

//A = 1 = Rock = X
//B = 2 = Paper = Y
//C = 3 = Scissors = Z

#[derive(PartialEq, Copy, Clone, Debug)]
enum Command {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Command {
    fn get_command(command: &str) -> Command {
        match command {
            "A" | "X" => {
                Rock
            }
            "B" | "Y" => {
                Paper
            }
            "C" | "Z" => {
                Scissors
            }
            _ => unreachable!()
        }
    }
}

fn beats(command: Command) -> Command {
    match command {
        Rock => Scissors,
        Paper => Rock,
        Scissors => Paper,
    }
}

fn loses(command: Command) -> Command {
    match command {
        Rock => Paper,
        Paper => Scissors,
        Scissors => Rock,
    }
}

fn results(command: Command, opponent_command: Command) -> i32 {
    return if command == opponent_command {
        3
    } else if beats(command) == opponent_command {
        6
    } else {
        0
    };
}



fn setup() -> BufReader<File> {
    let file = File::open("resources/day2.txt").expect("File not found");
    BufReader::new(file)
}

pub(crate) fn part1() {
    let reader = setup();
    let mut sum = 0;
    for line in reader.lines() {
        if let Ok(string_command) = line {
            if let Some(commands) = string_command.split_once(" ") {
                let (opponent_command, my_command) = (Command::get_command(commands.0), Command::get_command(commands.1));
                sum += results(my_command, opponent_command) + my_command as i32;
            }
        }
    }
    println!("Day 2 Part 1: {:?}", sum);
}

pub(crate) fn part2() {
    let reader = setup();
    let mut sum = 0;
    for line in reader.lines() {
        if let Ok(string_command) = line {
            if let Some(commands) = string_command.split_once(" ") {
                let (opponent_command, expected_results) = (Command::get_command(commands.0), commands.1);
                match expected_results {
                    "X" => {
                        //lose
                        sum += results(beats(opponent_command), opponent_command) + beats(opponent_command) as i32
                    }
                    "Y" => {
                        //draw
                        sum += results(opponent_command, opponent_command) + opponent_command as i32;
                    }
                    "Z" => {
                        //win
                        sum += results(loses(opponent_command), opponent_command) + loses(opponent_command) as i32
                    }
                    _ => unreachable!()
                };
            }
        }
    }
    println!("Day 2 Part 1: {:?}", sum);
}