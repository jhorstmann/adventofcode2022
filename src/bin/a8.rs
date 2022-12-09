use adventofcode2022::read_lines;
use adventofcode2022::Result;

fn main() -> Result<()> {
    let lines = read_lines("data/a8.txt")?;

    let width = lines.first().map(String::len).unwrap_or(0_usize);
    let height = lines.len();

    println!("{width}x{height}");

    let forest = lines.into_iter().flat_map(|line| line.into_bytes()).collect::<Vec<_>>();

    let mut seen = vec![false; width * height];

    let mut count = 0_usize;

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
    println!("part1: {count}");

    let mut max = 0;
    for row in 0..width {
        for col in 0..height {
            let i = row * width + col;
            if seen[i] {
                let mut a = 0;
                let mut b = 0;
                let mut c = 0;
                let mut d = 0;

                for x in (col + 1)..width {
                    a += 1;
                    let j = row * width + x;
                    if forest[j] >= forest[i] {
                        break;
                    }
                }
                for x in (0..col).rev() {
                    b += 1;
                    let j = row * width + x;
                    if forest[j] >= forest[i] {
                        break;
                    }
                }
                for y in (row + 1)..height {
                    c += 1;
                    let j = y * width + col;
                    if forest[j] >= forest[i] {
                        break;
                    }
                }
                for y in (0..row).rev() {
                    d += 1;
                    let j = y * width + col;
                    if forest[j] >= forest[i] {
                        break;
                    }
                }

                if row == 1 && col == 2 {
                    dbg!(a, b, c, d);
                }

                max = max.max(a * b * c * d);
            }
        }
    }

    println!("part2: {max}");

    Ok(())
}
