use adventofcode2022::Result;
use adventofcode2022::{read_lines, Bitmap64};
use std::collections::VecDeque;

fn main() -> Result<()> {
    let lines = read_lines("data/a6.txt")?;

    for line in &lines {
        println!("part1: {}", part1(&line).unwrap());
    }
    println!();

    for line in &lines {
        println!("part2: {}", part2(&line, 14).unwrap());
    }

    Ok(())
}

fn part1(line: &str) -> Option<usize> {
    let bytes = line.as_bytes();
    for (i, (((a, b), c), d)) in bytes
        .iter()
        .zip(bytes.iter().skip(1))
        .zip(bytes.iter().skip(2))
        .zip(bytes.iter().skip(3))
        .enumerate()
    {
        let mut bitmap = Bitmap64::default();
        bitmap.set_mut((*a - b'a') as usize);
        bitmap.set_mut((*b - b'a') as usize);
        bitmap.set_mut((c - b'a') as usize);
        bitmap.set_mut((*d - b'a') as usize);

        if bitmap.count_ones() == 4 {
            return Some(i + 4);
        }
    }
    None
}

fn part2(line: &str, n: usize) -> Option<usize> {
    let mut counters = vec![0; 256];
    let mut deque = VecDeque::default();

    line.as_bytes().iter().enumerate().find_map(|(i, b)| {
        deque.push_back(*b);
        counters[*b as usize] += 1;
        if deque.len() > 14 {
            let x = deque.pop_front().unwrap();
            counters[x as usize] -= 1;
        }
        if counters.iter().filter(|c| **c == 1).count() == n {
            Some(i + 1)
        } else {
            None
        }
    })
}
