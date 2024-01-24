pub fn process_part_1(input: &str) -> String {
    0.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "24");
    }
}
