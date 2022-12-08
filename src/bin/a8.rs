use adventofcode2022::read_lines;
use adventofcode2022::Result;
use std::num::ParseIntError;

fn main() -> Result<()> {
    let lines = read_lines("data/a8.txt")?;

    let width = lines.first().map(String::len).unwrap_or(0_usize);
    let height = lines.len();

    println!("{width}x{height}");

    let forest = lines.into_iter().flat_map(|line| line.into_bytes()).collect::<Vec<_>>();

    let mut seen = vec![false; width * height];

    let mut count = 0_usize;

    // const DIRS: [(usize, usize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for row in 0..height {
        let mut z = 0_u8;
        for col in 0..width {
            let i = row * width + col;
            if !seen[i] && forest[i] > z {
                count += 1;
                seen[i] = true;
            }
            z = z.max(forest[i]);
        }
    }

    for row in 0..height {
        let mut z = 0_u8;
        for col in (0..width).rev() {
            let i = row * width + col;
            if !seen[i] && forest[i] > z {
                count += 1;
                seen[i] = true;
            }
            z = z.max(forest[i]);
        }
    }

    for col in 0..width {
        let mut z = 0_u8;
        for row in 0..width {
            let i = row * width + col;
            if !seen[i] && forest[i] > z {
                count += 1;
                seen[i] = true;
            }
            z = z.max(forest[i]);
        }
    }
    for col in 0..height {
        let mut z = 0_u8;
        for row in (0..width).rev() {
            let i = row * width + col;
            if !seen[i] && forest[i] > z {
                count += 1;
                seen[i] = true;
            }
            z = z.max(forest[i]);
        }
    }

    for row in 0..width {
        for col in 0..height {
            let i = row * width + col;
            print!("{}", if seen[i] { 'x' } else { ' ' })
        }
        println!()
    }
    dbg!(count);

    Ok(())
}
