use std::collections::HashSet;
use std::hash::Hash;

fn intersection<T: Eq + Hash>(a: HashSet<T>, b: &HashSet<T>) -> HashSet<T> {
    a.into_iter().filter(|e| b.contains(e)).collect()
}

fn split(s: &str) -> (HashSet<char>, HashSet<char>) {
    let length = s.len();
    (
        s[..length / 2].chars().collect::<HashSet<char>>(),
        s[length / 2..].chars().collect::<HashSet<char>>(),
    )
}

fn calculate_cost(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u8 - 'A' as u8 + 27) as u32
    } else {
        (c as u8 - 'a' as u8 + 1) as u32
    }
}

pub fn process_part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| split(line))
        .map(|t| intersection(t.0, &t.1))
        .fold(vec![], |mut acc: Vec<char>, i: HashSet<char>| {
            acc.extend(i.into_iter());
            acc
        })
        .iter()
        .map(|c| calculate_cost(*c))
        .sum::<u32>()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|ch| {
            ch.iter()
                .map(|c| (*c).chars().collect::<HashSet<char>>())
                .reduce(|acc, i| intersection(acc, &i))
                .unwrap()
        })
        .fold(vec![], |mut acc: Vec<char>, i: HashSet<char>| {
            acc.extend(i.into_iter());
            acc
        })
        .iter()
        .map(|c| calculate_cost(*c))
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
}
