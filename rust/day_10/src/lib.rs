use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
    ops::RangeInclusive,
};

struct Computer {
    x: i32,
    cycle: u32,
    pixels: String,
}

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.pixels
                .chars()
                .chunks(40)
                .into_iter()
                .map(|chunk| chunk.collect::<String>())
                .join("\n")
        )
    }
}

impl Computer {
    fn new() -> Self {
        Computer {
            x: 1,
            cycle: 0,
            pixels: "".to_string(),
        }
    }

    fn sprite_range(&self) -> RangeInclusive<i32> {
        (self.x - 1)..=(self.x + 1)
    }

    fn interpret(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.wait() {
            let cycle_guard = self.start_cycle();

            if cycle_guard
                .computer
                .sprite_range()
                .contains(&(cycle_guard.pixel as i32))
            {
                cycle_guard.computer.pixels.push_str("#");
            } else {
                cycle_guard.computer.pixels.push_str(".");
            }
        }

        match instruction {
            Noop => {}
            Addx(num) => {
                self.x += num;
            }
        };
    }

    fn start_cycle(&mut self) -> Cycle {
        Cycle {
            cycle: self.cycle,
            pixel: self.cycle % 40,
            computer: self,
        }
    }
}

struct Cycle<'a> {
    cycle: u32,
    pixel: u32,
    computer: &'a mut Computer,
}
impl<'a> Drop for Cycle<'a> {
    fn drop(&mut self) {
        self.computer.cycle += 1;
    }
}

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

use Instruction::*;

impl Instruction {
    fn wait(&self) -> i32 {
        match self {
            Noop => 1,
            Addx(_) => 2,
        }
    }
}

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;

    return Ok((input, Noop));
}

fn parse_addx(input: &str) -> IResult<&str, Instruction> {
    let (input, number) = preceded(tag("addx "), complete::i32)(input)?;

    return Ok((input, Addx(number)));
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = separated_list1(newline, alt((parse_noop, parse_addx)))(input)?;

    Ok((input, instructions))
}

pub fn process_part_1(input: &str) -> String {
    let (_, instructions) = parse_instructions(input).unwrap();
    let notable_cycles: BTreeSet<u32> = [20, 60, 100, 140, 180, 220].into_iter().collect();
    let mut scores: BTreeMap<u32, i32> = BTreeMap::new();
    let mut computer = Computer::new();

    for instruction in instructions.iter() {
        if notable_cycles.contains(&(computer.cycle + 1)) {
            scores.insert(
                (computer.cycle + 1) as u32,
                (computer.cycle as i32 + 1) * computer.x,
            );
        }

        if notable_cycles.contains(&(computer.cycle + 2)) {
            scores.insert(
                (computer.cycle + 2) as u32,
                (computer.cycle as i32 + 2) * computer.x,
            );
        }

        computer.cycle += instruction.wait() as u32;
        match instruction {
            Noop => {}
            Addx(num) => {
                computer.x += num;
            }
        };
    }

    scores
        .iter()
        .map(|(_key, value)| value)
        .sum::<i32>()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (_, instructions) = parse_instructions(input).unwrap();

    instructions
        .iter()
        .fold(Computer::new(), |mut computer, instruction| {
            computer.interpret(instruction);
            computer
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "13140");
    }

    const OUTPUT_PART_2: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, OUTPUT_PART_2);
    }
}
