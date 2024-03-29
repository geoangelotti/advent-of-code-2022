use ::lending_iterator::prelude::*;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn direction(input: &str) -> IResult<&str, Direction> {
    let (input, dir) = alt((
        complete::char('L').map(|_| Direction::Left),
        complete::char('R').map(|_| Direction::Right),
        complete::char('U').map(|_| Direction::Up),
        complete::char('D').map(|_| Direction::Down),
    ))(input)?;
    Ok((input, dir))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, vecs) =
        separated_list1(newline, separated_pair(direction, tag(" "), complete::u32))(input)?;

    let vecs = vecs
        .iter()
        .flat_map(|(dir, repeat)| vec![*dir; *repeat as usize])
        .collect();
    Ok((input, vecs))
}

pub fn process_part_1(input: &str) -> String {
    let (_, moves) = parse_moves(input).unwrap();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_positions = HashSet::from([tail]);
    for head_move in moves.iter() {
        match head_move {
            Direction::Left => head.0 -= 1,
            Direction::Right => head.0 += 1,
            Direction::Up => head.1 += 1,
            Direction::Down => head.1 -= 1,
        }
        let x_range = (head.0 - 1)..=(head.0 + 1);
        let y_range = (head.1 - 1)..=(head.1 + 1);

        let tail_in_range = x_range
            .cartesian_product(y_range)
            .any(|point| point == tail);

        if !tail_in_range {
            tail = head.clone();
            match head_move {
                Direction::Left => tail.0 += 1,
                Direction::Right => tail.0 -= 1,
                Direction::Up => tail.1 -= 1,
                Direction::Down => tail.1 += 1,
            }
            tail_positions.insert(tail);
        }
    }
    tail_positions.len().to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (_, moves) = parse_moves(input).unwrap();
    let mut rope = vec![(0, 0); 10];
    let mut tail_positions = HashSet::from([*rope.last().unwrap()]);
    for head_move in moves.iter() {
        match head_move {
            Direction::Left => rope[0].0 -= 1,
            Direction::Right => rope[0].0 += 1,
            Direction::Up => rope[0].1 += 1,
            Direction::Down => rope[0].1 -= 1,
        }

        let mut rope_windows = rope.windows_mut::<2>();
        while let Some([ref mut head, ref mut tail]) = rope_windows.next() {
            let x_range = (head.0 - 1)..=(head.0 + 1);
            let y_range = (head.1 - 1)..=(head.1 + 1);

            let tail_is_connected = x_range
                .cartesian_product(y_range)
                .any(|tuple| tuple == *tail);

            if !tail_is_connected {
                if head.0 == tail.0 {
                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else {
                        tail.1 -= 1;
                    }
                } else if head.1 == tail.1 {
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else {
                        tail.0 -= 1;
                    }
                } else {
                    let x_range = (head.0 - 1)..=(head.0 + 1);
                    let y_range = (head.1 - 1)..=(head.1 + 1);

                    let head_3x3 = x_range.cartesian_product(y_range).collect::<Vec<_>>();

                    let x_range = (tail.0 - 1)..=(tail.0 + 1);
                    let y_range = (tail.1 - 1)..=(tail.1 + 1);

                    let maybe_new_tail: Vec<(i32, i32)> = x_range
                        .cartesian_product(y_range)
                        .filter(|tuple| head_3x3.contains(tuple))
                        .collect();
                    match maybe_new_tail.len() {
                        2 => {
                            let new_head_cross_positions = [
                                (head.0 - 1, head.1),
                                (head.0 + 1, head.1),
                                (head.0, head.1 - 1),
                                (head.0, head.1 + 1),
                            ];
                            let next = maybe_new_tail
                                .iter()
                                .find(|tuple| new_head_cross_positions.contains(tuple))
                                .unwrap();
                            *tail = *next;
                        }
                        1 => {
                            *tail = maybe_new_tail[0];
                        }
                        _ => {
                            panic!("unknown tail length");
                        }
                    };
                }
            }
        }
        tail_positions.insert(*rope.last().unwrap());
    }
    tail_positions.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(
            process_part_1(
                "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
            ),
            "13"
        );
    }

    #[test]
    fn part_2_works() {
        assert_eq!(
            process_part_2(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            ),
            "36"
        );
    }
}
