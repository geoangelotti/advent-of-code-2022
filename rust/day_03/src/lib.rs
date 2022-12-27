use std::collections::HashSet;
use std::hash::Hash;

pub fn intersection<T: Eq + Hash>(a: HashSet<T>, b: &HashSet<T>) -> HashSet<T> {
    a.into_iter().filter(|e| b.contains(e)).collect()
}

pub fn split(s: &str) -> (HashSet<char>, HashSet<char>) {
    let length = s.len();
    (
        s[..length / 2].chars().collect::<HashSet<char>>(),
        s[length / 2..].chars().collect::<HashSet<char>>(),
    )
}

pub fn calculate_cost(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u8 - 'A' as u8 + 27) as u32
    } else {
        (c as u8 - 'a' as u8 + 1) as u32
    }
}

pub fn do_(lines: Vec<String>) {
    let count1: u32 = lines
        .iter()
        .map(|line| split(line.as_str()))
        .map(|t| intersection(t.0, &t.1))
        .fold(vec![], |mut acc: Vec<char>, i: HashSet<char>| {
            acc.extend(i.into_iter());
            acc
        })
        .iter()
        .map(|c| calculate_cost(*c))
        .sum();

    let count2: u32 = lines
        .chunks(3)
        .map(|ch| {
            ch.iter()
                .map(|c| c.chars().collect::<HashSet<char>>())
                .reduce(|acc, i| intersection(acc, &i))
                .unwrap()
        })
        .fold(vec![], |mut acc: Vec<char>, i: HashSet<char>| {
            acc.extend(i.into_iter());
            acc
        })
        .iter()
        .map(|c| calculate_cost(*c))
        .sum();

    println!("{:?}", count1);
    println!("{:?}", count2);
}

#[cfg(test)]
mod tests {
    use super::*;
}
