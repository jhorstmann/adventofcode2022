use adventofcode2022::Result;
use adventofcode2022::{read_lines, Bitmap64};

fn item_value(item: u8) -> usize {
    match item {
        b'a'..=b'z' => item as usize - b'a' as usize + 1,
        b'A'..=b'Z' => item as usize - b'A' as usize + 27,
        _ => unreachable!(),
    }
}

fn main() -> Result<()> {
    let lines = read_lines("data/a3.txt")?;

    let part1 = lines
        .iter()
        .map(|line| {
            let (head, tail) = line.split_at(line.len() / 2);
            let a = Bitmap64::from_iter(head.bytes().map(item_value));
            let b = Bitmap64::from_iter(tail.bytes().map(item_value));
            let c = a.and(&b);
            c.as_u64().trailing_zeros() as u64
        })
        .sum::<u64>();

    println!("part1: {part1}");

    let part2 = lines
        .chunks(3)
        .map(|chunk| {
            let common = chunk.iter().fold(Bitmap64::from(u64::MAX), |acc, line| {
                let tmp = Bitmap64::from_iter(line.bytes().map(item_value));
                acc.and(&tmp)
            });

            common.as_u64().trailing_zeros() as u64
        })
        .sum::<u64>();

    println!("part2: {part2}");

    Ok(())
}
