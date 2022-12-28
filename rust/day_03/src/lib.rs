#![feature(iter_array_chunks)]

use std::collections::HashMap;

pub fn process_part_1(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    input
        .lines()
        .map(|line| {
            let midpoint = line.len() / 2;
            let compartment_1 = &line[..midpoint];
            let compartment_2 = &line[midpoint..];
            let common_letter = compartment_1
                .chars()
                .find(|c| compartment_2.contains(*c))
                .unwrap();
            letter_scores.get(&common_letter).unwrap()
        })
        .sum::<usize>()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let common_letter = a
                .chars()
                .find(|a_char| b.contains(*a_char) && c.contains(*a_char))
                .unwrap();
            letter_scores.get(&common_letter).unwrap()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "70");
    }
}
