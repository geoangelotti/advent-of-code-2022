use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};

#[derive(Debug, Clone, Copy)]
enum Value {
    Old,
    Num(u64),
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Mul((Value, Value)),
    Add((Value, Value)),
}

#[derive(Debug)]
struct Test {
    divisible: u64,
    true_recipient: u64,
    false_recipient: u64,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    touch_count: u64,
}

impl Monkey {
    fn inspect(&mut self, relief_lowers_worry_level: bool) -> u64 {
        self.touch_count += 1;
        let item = self.items.pop_front().unwrap();
        let worry_level = match &self.operation {
            Operation::Mul((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                let num_b = match b {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                let result = num_a * num_b;
                result
            }
            Operation::Add((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                let num_b = match b {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                let result = num_a + num_b;
                result
            }
        };
        let result = if relief_lowers_worry_level {
            worry_level / 3
        } else {
            worry_level
        };
        result
    }

    fn test(&self, item: u64) -> u64 {
        if item % self.test.divisible == 0 {
            self.test.true_recipient
        } else {
            self.test.false_recipient
        }
    }
}

fn value(input: &str) -> IResult<&str, Value> {
    alt((
        tag("old").map(|_| Value::Old),
        nom::character::complete::u64.map(|num| Value::Num(num)),
    ))(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("Operation: new = ")(input)?;
    let (input, value_1) = value(input)?;
    let (input, operator) = delimited(multispace1, alt((tag("*"), tag("+"))), multispace1)(input)?;
    let (input, value_2) = value(input)?;

    let result = match operator {
        "*" => Operation::Mul((value_1, value_2)),
        "+" => Operation::Add((value_1, value_2)),
        _ => panic!("unknown operator"),
    };
    Ok((input, result))
}

fn test(input: &str) -> IResult<&str, Test> {
    let (input, divisible) =
        preceded(tag("Test: divisible by "), nom::character::complete::u64)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, true_recipient) = preceded(
        tag("If true: throw to monkey "),
        nom::character::complete::u64,
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, false_recipient) = preceded(
        tag("If false: throw to monkey "),
        nom::character::complete::u64,
    )(input)?;
    Ok((
        input,
        Test {
            divisible,
            true_recipient,
            false_recipient,
        },
    ))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _id) = delimited(tag("Monkey "), nom::character::complete::u64, tag(":"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), nom::character::complete::u64),
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, op) = operation(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test) = test(input)?;

    Ok((
        input,
        Monkey {
            items: VecDeque::from(items),
            operation: op,
            test,
            touch_count: 0,
        },
    ))
}

pub fn process_part_1(input: &str) -> String {
    let (_, mut monkeys) = separated_list1(tag("\n\n"), monkey)(input).unwrap();
    for _ in 0..20 {
        for monkey_index in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_index].items.len() {
                let monkey = monkeys.get_mut(monkey_index).unwrap();
                let item = monkey.inspect(true);
                let monkey_to_send_to = monkey.test(item);
                monkeys
                    .get_mut(monkey_to_send_to as usize)
                    .unwrap()
                    .items
                    .push_back(item);
            }
        }
    }
    monkeys.sort_by_key(|monkey| monkey.touch_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.touch_count)
        .product::<u64>()
        .to_string()
}

pub fn process_part_2(_input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3

Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0

Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3

Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
        If true: throw to monkey 0
        If false: throw to monkey 1";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "10605");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2("");
        assert_eq!(result, "");
    }
}
