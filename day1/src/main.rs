use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_input(path: &str) -> BufReader<File> {
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

fn get_calories(reader: BufReader<File>) -> Vec<u64> {
    let mut calories: Vec<u64> = Vec::new();
    let mut acc = 0u64;

    for line in reader.lines().map(|l| l.unwrap()) {
        match line.as_str() {
            "" => {
                calories.push(acc);
                acc = 0;
            }
            _ => {
                acc += line.trim().parse::<u64>().unwrap();
            }
        };
    }
    calories
}

fn part1(calories: &Vec<u64>) {
    let max_calories = match calories.to_vec().pop() {
        Some(c) => c,
        None => 0,
    };
    println!("{:?}", max_calories)
}

fn part2(calories: &Vec<u64>) {
    let mut calories = calories.to_vec();
    let max_calories = match (calories.pop(), calories.pop(), calories.pop()) {
        (Some(a), Some(b), Some(c)) => a + b + c,
        (Some(a), Some(b), None) => a + b,
        (Some(a), None, None) => a,
        _ => 0,
    };
    println!("{}", max_calories)
}

fn main() {
    let reader = read_input("input.txt");
    let mut calories = get_calories(reader);
    calories.sort();
    part1(&calories);
    part2(&calories);
}
