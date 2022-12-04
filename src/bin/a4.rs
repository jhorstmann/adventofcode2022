use adventofcode2022::read_lines;
use adventofcode2022::Result;
use std::ops::RangeInclusive;

fn parse_range(range: &str) -> RangeInclusive<usize> {
    let mut split = range.splitn(2, "-");
    let start = split.next().unwrap().parse::<usize>().unwrap();
    let end = split.next().unwrap().parse::<usize>().unwrap();
    start..=end
}

fn fully_contains(first: &RangeInclusive<usize>, second: &RangeInclusive<usize>) -> bool {
    first.contains(&second.start()) && first.contains(&second.end())
}

fn overlap(first: &RangeInclusive<usize>, second: &RangeInclusive<usize>) -> bool {
    first.contains(&second.start()) || first.contains(&second.end())
}

fn main() -> Result<()> {
    let lines = read_lines("data/a4.txt")?;
    let pairs: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> = lines
        .iter()
        .map(|line: &String| {
            let mut split = line.splitn(2, ",");
            let first = parse_range(split.next().unwrap());
            let second = parse_range(split.next().unwrap());
            (first, second)
        })
        .collect::<Vec<_>>();

    // dbg!(&pairs);

    let part1 = pairs
        .iter()
        .filter(|(first, second)| fully_contains(first, second) || fully_contains(second, first))
        .count();

    println!("part1: {part1}");

    let part2 = pairs
        .iter()
        .filter(|(first, second)| overlap(first, second) || overlap(second, first))
        .count();

    println!("part2: {part2}");

    Ok(())
}
