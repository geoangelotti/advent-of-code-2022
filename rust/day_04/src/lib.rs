use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn sections(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, (start, end)) = separated_pair(complete::u32, tag("-"), complete::u32)(input)?;
    Ok((input, start..=end))
}
fn line(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (input, (start, end)) = separated_pair(sections, tag(","), sections)(input)?;

    Ok((input, (start, end)))
}
fn section_assignments(
    input: &str,
) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    let (input, ranges) = separated_list1(newline, line)(input)?;

    Ok((input, ranges))
}

pub fn process_part_1(input: &str) -> String {
    let (_, assignments) = section_assignments(input).unwrap();

    assignments
        .iter()
        .filter(|(range_a, range_b)| {
            let a_contains_b = range_a
                .clone()
                .into_iter()
                .all(|num| range_b.contains(&num));
            let b_contains_a = range_b
                .clone()
                .into_iter()
                .all(|num| range_a.contains(&num));
            a_contains_b || b_contains_a
        })
        .count()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (_, assignments) = section_assignments(input).unwrap();

    assignments
        .iter()
        .filter(|(range_a, range_b)| {
            let a_contains_b = range_a
                .clone()
                .into_iter()
                .any(|num| range_b.contains(&num));
            let b_contains_a = range_b
                .clone()
                .into_iter()
                .any(|num| range_a.contains(&num));
            a_contains_b || b_contains_a
        })
        .count()
        .to_string()
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
