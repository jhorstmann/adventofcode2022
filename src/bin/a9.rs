use adventofcode2022::Result;
use adventofcode2022::{read_lines, Error};
use std::cell::Cell;
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

    println!("part1: {}", solve(&lines, 2));
    println!("part2: {}", solve(&lines, 10));

    Ok(())
}

fn solve(lines: &[(char, usize)], n: usize) -> usize {
    let mut visited = HashSet::new();
    visited.insert((0_i64, 0_i64));
    let init = vec![(0_i64, 0_i64); n];
    lines.iter().fold(init, |mut knots, (dir, steps)| {
        for _ in 0..*steps {
            let (ref mut hx, ref mut hy) = knots[0];
            match dir {
                'U' => {
                    *hy += 1;
                }
                'D' => {
                    *hy -= 1;
                }
                'L' => {
                    *hx -= 1;
                }
                'R' => {
                    *hx += 1;
                }
                _ => unreachable!(),
            }

            let knot_cells = Cell::from_mut(knots.as_mut_slice()).as_slice_of_cells();
            for w in knot_cells.windows(2) {
                let (hx, hy) = Cell::get(&w[0]);
                let (mut tx, mut ty) = Cell::get(&w[1]);

                if (ty.abs_diff(hy) > 1) ^ (tx.abs_diff(hx) > 1) {
                    ty -= (ty - hy).signum();
                    tx -= (tx - hx).signum();
                } else if (ty.abs_diff(hy) > 1) && (tx.abs_diff(hx) > 1) {
                    ty -= (ty - hy).signum();
                    tx -= (tx - hx).signum();
                }

                assert!(hx.abs_diff(tx) <= 1);
                assert!(hy.abs_diff(ty) <= 1);

                Cell::set(&w[0], (hx, hy));
                Cell::set(&w[1], (tx, ty));
            }
            visited.insert(knots[n - 1]);
        }

        knots
    });
    visited.len()
}
