use adventofcode2022::Result;
use adventofcode2022::{read_lines, Error};
use std::collections::HashSet;

fn main() -> Result<()> {
    let lines = read_lines("data/a9.txt")?
        .into_iter()
        .map(|line| {
            let mut split = line.split(" ");
            let dir = split
                .next()
                .ok_or(Error::EmptyIterator)?
                .chars()
                .next()
                .ok_or(Error::EmptyIterator)?;
            let steps = split.next().ok_or(Error::EmptyIterator)?.parse::<usize>()?;
            Ok((dir, steps))
        })
        .collect::<Result<Vec<(char, usize)>>>()?;

    println!("part1: {}", solve_part1(lines));

    Ok(())
}

fn solve_part1(lines: Vec<(char, usize)>) -> usize {
    let mut visited = HashSet::new();
    lines.iter().fold(
        (0_i64, 0_i64, 0_i64, 0_i64),
        |(mut hx, mut hy, mut tx, mut ty), (dir, steps)| {
            for _ in 0..*steps {
                // dbg!(hx, hy, tx, ty);
                assert!(hx.abs_diff(tx) <= 1);
                assert!(hy.abs_diff(ty) <= 1);
                match dir {
                    'U' => {
                        hy += 1;
                        if ty.abs_diff(hy) > 1 {
                            ty += 1;
                            if tx != hx {
                                tx = hx;
                            }
                        }
                    }
                    'D' => {
                        hy -= 1;
                        if ty.abs_diff(hy) > 1 {
                            ty -= 1;
                            if tx != hx {
                                tx = hx;
                            }
                        }
                    }
                    'L' => {
                        hx -= 1;
                        if tx.abs_diff(hx) > 1 {
                            tx -= 1;
                            if ty != hy {
                                ty = hy;
                            }
                        }
                    }
                    'R' => {
                        hx += 1;
                        if tx.abs_diff(hx) > 1 {
                            tx += 1;
                            if ty != hy {
                                ty = hy;
                            }
                        }
                    }
                    _ => unreachable!(),
                }
                visited.insert((tx, ty));
            }
            (hx, hy, tx, ty)
        },
    );
    visited.len()
}
