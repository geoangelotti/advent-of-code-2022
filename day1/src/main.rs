use advent::utils::get_lines;

fn get_calories(path: &str) -> Vec<u64> {
    let mut calories: Vec<u64> = Vec::new();
    let mut acc = 0u64;

    for line in get_lines(path).iter() {
        match line.as_str() {
            "" => {
                calories.push(acc);
                acc = 0;
            }
            _ => {
                acc += line.trim().parse::<u64>().unwrap();
            }
        };
    }
    calories
}

fn part1(calories: &Vec<u64>) {
    let max_calories = match calories.to_vec().pop() {
        Some(c) => c,
        None => 0,
    };
    println!("{:?}", max_calories)
}

fn part2(calories: &Vec<u64>) {
    let mut calories = calories.to_vec();
    let max_calories = match (calories.pop(), calories.pop(), calories.pop()) {
        (Some(a), Some(b), Some(c)) => a + b + c,
        (Some(a), Some(b), None) => a + b,
        (Some(a), None, None) => a,
        _ => 0,
    };
    println!("{}", max_calories)
}

fn main() {
    let mut calories = get_calories("input.txt");
    calories.sort();
    part1(&calories);
    part2(&calories);
}
