#![feature(iter_intersperse)]
use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
enum Command<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}
#[derive(Debug)]
enum Files<'a> {
    File { size: u32, name: &'a str },
    Dir(&'a str),
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, (size, name)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        is_a("qwertyuiopasdfghjklzxcvbnm."),
    )(input)?;
    Ok((input, Files::File { size, name }))
}

fn directory(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, Files::Dir(name)))
}

fn ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((file, directory)))(input)?;
    Ok((input, Command::Ls(files)))
}

fn cd(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;
    let op = match dir {
        "/" => Command::Cd(Cd::Root),
        ".." => Command::Cd(Cd::Up),
        name => Command::Cd(Cd::Down(name)),
    };
    Ok((input, op))
}

fn commands(input: &str) -> IResult<&str, Vec<Command>> {
    let (input, cmd) = separated_list1(newline, alt((ls, cd)))(input)?;

    Ok((input, cmd))
}

fn calculate_sizes<'a>(
    (mut context, mut sizes): (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>),
    command: &'a Command,
) -> (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>) {
    match command {
        Command::Cd(Cd::Root) => {
            context.push("");
        }
        Command::Cd(Cd::Up) => {
            context.pop();
        }
        Command::Cd(Cd::Down(name)) => {
            context.push(name);
        }
        Command::Ls(files) => {
            let sum = files
                .iter()
                .filter_map(|file| {
                    if let Files::File { size, .. } = file {
                        Some(size)
                    } else {
                        None
                    }
                })
                .sum::<u32>();

            for i in 0..context.len() {
                sizes
                    .entry(context[0..=i].to_vec())
                    .and_modify(|v| *v += sum)
                    .or_insert(sum);
            }
        }
    };
    (context, sizes)
}

pub fn process_part_1(input: &str) -> String {
    let (_, cmds) = commands(input).unwrap();

    let (_, sizes) = cmds.iter().fold((vec![], BTreeMap::new()), calculate_sizes);

    sizes
        .iter()
        .filter(|(_, &size)| size < 100_000)
        .map(|(_, size)| size)
        .sum::<u32>()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (_, cmds) = commands(input).unwrap();
    let (_, sizes) = cmds.iter().fold((vec![], BTreeMap::new()), calculate_sizes);

    let total_size = 70_000_000;
    let needed_space = 30_000_000;

    let used_space = sizes.get(&vec![""]).unwrap();

    let current_free_space = total_size - used_space;
    let need_to_free_at_least = needed_space - current_free_space;

    let mut valid_dirs = sizes
        .iter()
        .filter(|(_, &size)| size > need_to_free_at_least)
        .map(|(_, size)| size)
        .collect::<Vec<&u32>>();

    valid_dirs.sort();
    valid_dirs.iter().next().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part_1_works() {
        assert_eq!(process_part_1(INPUT), "95437");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(process_part_2(INPUT), "24933642");
    }
}
