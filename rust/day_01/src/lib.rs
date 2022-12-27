pub fn process_part_1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|calories_chunk| {
            calories_chunk
                .lines()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    let mut result = input
        .split("\n\n")
        .map(|calories_chunk| {
            calories_chunk
                .lines()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    result.sort_by(|a, b| b.cmp(a));
    result.iter().take(3).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, 24000.to_string());
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, 45000.to_string());
    }
}
