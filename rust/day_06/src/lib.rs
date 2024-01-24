use std::collections::BTreeSet;

fn process_windows(input: &str, window_size: usize) -> String {
    (input
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .enumerate()
        .find(|(_, slice)| {
            let set = slice.iter().collect::<BTreeSet<&char>>();
            slice.len() == set.len()
        })
        .unwrap()
        .0
        + window_size)
        .to_string()
}

pub fn process_part_1(input: &str) -> String {
    process_windows(input, 4)
}

pub fn process_part_2(input: &str) -> String {
    process_windows(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(process_part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "7");
        assert_eq!(process_part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), "5");
        assert_eq!(process_part_1("nppdvjthqldpwncqszvftbrmjlhg"), "6");
        assert_eq!(process_part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "10");
        assert_eq!(process_part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "11");
    }

    #[test]
    fn part_2_works() {
        //qmgbljsphdztnv
        assert_eq!(process_part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "19");
        assert_eq!(process_part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), "23");
        assert_eq!(process_part_2("nppdvjthqldpwncqszvftbrmjlhg"), "23");
        assert_eq!(process_part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "29");
        assert_eq!(process_part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "26");
    }
}
