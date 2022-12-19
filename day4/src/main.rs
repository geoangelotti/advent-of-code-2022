use advent::utils::get_lines;

fn overlaps<T>(lines: &Vec<String>, limits: T) -> u32
where
    T: Fn((u32, u32, u32, u32)) -> bool,
{
    lines
        .iter()
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
        .sum()
}

fn main() {
    let lines = &get_lines("input.txt");

    println!(
        "{}",
        overlaps(lines, |tuple| {
            (tuple.0 <= tuple.2 && tuple.3 <= tuple.1) || (tuple.2 <= tuple.0 && tuple.1 <= tuple.3)
        })
    );

    println!(
        "{}",
        overlaps(lines, |tuple| {
            !((tuple.1 < tuple.2) || (tuple.3 < tuple.0))
        })
    );
}
