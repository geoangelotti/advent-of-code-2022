use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use std::cmp::Ordering::{self, *};

#[derive(Debug, PartialEq)]
pub struct Pair {
    left: Packet,
    right: Packet,
}
#[derive(Debug, Eq)]
pub enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::List(l0), Self::Number(r0)) => l0 == &vec![Packet::Number(*r0)],
            (Self::Number(l0), Self::List(r0)) => &vec![Packet::Number(*l0)] == r0,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::List(a), Packet::Number(b)) => a.cmp(&vec![Packet::Number(*b)]),
            (Packet::Number(a), Packet::List(b)) => vec![Packet::Number(*a)].cmp(&b),
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
        }
    }
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), packet), tag("]"))
            .map(|vec| Packet::List(vec)),
        nom::character::complete::u32.map(|num| Packet::Number(num)),
    ))(input)
}

fn pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(packet, newline, packet).map(|(p1, p2)| Pair {
            left: p1,
            right: p2,
        }),
    )(input)
}

pub fn process_part_1(input: &str) -> String {
    let (_, pairs) = pairs(input).unwrap();
    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, Pair { left, right })| match left.cmp(right) {
            Less => Some(i),
            Equal => panic!("equal??"),
            Greater => None,
        })
        .map(|v| v + 1)
        .sum::<usize>()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (_, pairs) = pairs(input).unwrap();
    let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let divider_6 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    let mut packets = pairs
        .iter()
        .flat_map(|Pair { left, right }| [left, right])
        .chain([&divider_2, &divider_6])
        .collect::<Vec<&Packet>>();
    packets.sort();
    let index_2 = packets
        .iter()
        .enumerate()
        .find(|(_, &packet)| packet == &divider_2)
        .unwrap();
    let index_6 = packets
        .iter()
        .enumerate()
        .find(|(_, &packet)| packet == &divider_6)
        .unwrap();
    ((index_2.0 + 1) * (index_6.0 + 1)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "140");
    }
}
