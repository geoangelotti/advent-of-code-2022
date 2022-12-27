use std::{cmp::Ordering, str::FromStr};

#[derive(PartialEq, Copy, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err("Not a known hand".to_string()),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Hand::Scissors && other == &Hand::Rock {
            Some(Ordering::Less)
        } else if self == &Hand::Rock && other == &Hand::Scissors {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

pub fn process_part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let game = line
                .split(" ")
                .map(|s| s.parse::<Hand>().unwrap())
                .collect::<Vec<Hand>>();

            match game[0].partial_cmp(&game[1]) {
                Some(Ordering::Equal) => 3 + game[1] as u32,
                Some(Ordering::Less) => 6 + game[1] as u32,
                Some(Ordering::Greater) => 0 + game[1] as u32,
                None => {
                    panic!("moves should be comparable")
                }
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let split = line.split(" ").collect::<Vec<&str>>();
            let opponent = split[0].parse::<Hand>().unwrap();
            match split[1] {
                "X" => {
                    0 + match opponent {
                        Hand::Rock => Hand::Scissors,
                        Hand::Paper => Hand::Rock,
                        Hand::Scissors => Hand::Paper,
                    } as u32
                }
                "Y" => 3 + opponent as u32,
                "Z" => {
                    6 + match opponent {
                        Hand::Rock => Hand::Paper,
                        Hand::Paper => Hand::Scissors,
                        Hand::Scissors => Hand::Rock,
                    } as u32
                }
                _ => panic!("Unexpected response"),
            }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_it_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn part2_it_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "12");
    }
}
