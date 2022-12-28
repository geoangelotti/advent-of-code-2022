fn overlaps<T>(input: &str, limits: T) -> u32
where
    T: Fn((u32, u32, u32, u32)) -> bool,
{
    input
        .lines()
        .map(|line| {
            let mut sections = line.split(",");
            let mut section_1 = sections.next().unwrap().split("-");
            let mut section_2 = sections.next().unwrap().split("-");
            let (s1_start, s1_end) = (
                section_1.next().unwrap().parse::<u32>().unwrap(),
                section_1.next().unwrap().parse::<u32>().unwrap(),
            );
            let (s2_start, s2_end) = (
                section_2.next().unwrap().parse::<u32>().unwrap(),
                section_2.next().unwrap().parse::<u32>().unwrap(),
            );
            (s1_start, s1_end, s2_start, s2_end)
        })
        .map(|tuple| limits(tuple))
        .map(|s| if s { 1 } else { 0 })
        .sum::<u32>()
}

pub fn process_part_1(input: &str) -> String {
    overlaps(input, |tuple| {
        (tuple.0 <= tuple.2 && tuple.3 <= tuple.1) || (tuple.2 <= tuple.0 && tuple.1 <= tuple.3)
    })
    .to_string()
}

pub fn process_part_2(input: &str) -> String {
    overlaps(input, |tuple| !((tuple.1 < tuple.2) || (tuple.3 < tuple.0))).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "4");
    }
}
