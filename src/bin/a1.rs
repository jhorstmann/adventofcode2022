use adventofcode2022::read_lines;
use adventofcode2022::Result;
use std::num::ParseIntError;

type ParseIntResult<T> = std::result::Result<T, ParseIntError>;

fn main() -> Result<()> {
    let lines = read_lines("data/a1.txt")?;
    let chunks = lines.split(String::is_empty).collect::<Vec<_>>();
    let mut sums = chunks
        .iter()
        .map(|chunk| {
            chunk
                .iter()
                .map(|x: &String| x.parse::<i32>())
                .sum::<ParseIntResult<i32>>()
        })
        .collect::<ParseIntResult<Vec<i32>>>()?;
    let max = sums.iter().copied().max().unwrap_or_default();

    println!("part1: {max}");

    sums.sort();
    sums.reverse();

    let max3 = sums.iter().copied().take(3).sum::<i32>();
    println!("part2: {max3}");

    Ok(())
}
