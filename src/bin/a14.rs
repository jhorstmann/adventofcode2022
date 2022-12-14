use adventofcode2022::{read_lines, Result};
use std::str::from_utf8;

pub fn main() -> Result<()> {
    let paths: Vec<Vec<(usize, usize)>> = read_lines("data/a14_example.txt")?
        .into_iter()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| {
                    let mut split = pair.splitn(2, ',');
                    let x = split.next().unwrap().parse().unwrap();
                    let y = split.next().unwrap().parse().unwrap();
                    (x, y)
                })
                .collect()
        })
        .collect();

    // dbg!(&paths);

    let (min_x, max_x, min_y, max_y) = paths.iter().flatten().copied().chain(std::iter::once((500, 0))).fold(
        (usize::MAX, usize::MIN, usize::MAX, usize::MIN),
        |mut acc, p| {
            acc.0 = acc.0.min(p.0);
            acc.1 = acc.1.max(p.0);
            acc.2 = acc.2.min(p.1);
            acc.3 = acc.3.max(p.1);
            acc
        },
    );

    dbg!(min_x, max_x, min_y, max_y);

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut cave = vec![b'.'; width * height];

    paths.iter().for_each(|path| {
        path.windows(2).for_each(|segment| {
            let (sx, sy) = segment[0];
            let (tx, ty) = segment[1];

            let dx = (tx as isize - sx as isize).signum();
            let dy = (ty as isize - sy as isize).signum();

            let (mut x, mut y) = (sx, sy);
            loop {
                cave[(((y - min_y) * width) + x - min_x as usize)] = b'#';
                if x == tx && y == ty {
                    break;
                }
                x = (x as isize + dx) as usize;
                y = (y as isize + dy) as usize;
            }
        })
    });

    // print_cave(&cave, width);

    println!("part1: {}", part1(cave.clone(), 500 - min_x, 0, width));

    Ok(())
}

fn part1(mut cave: Vec<u8>, sx: usize, sy: usize, width: usize) -> usize {
    assert_eq!(cave.len() % width, 0);

    let height = (cave.len() / width) as isize;
    let width = width as isize;
    let mut count = 0_usize;
    'outer: loop {
        let mut x = sx as isize;
        let mut y = sy as isize;

        count += 1;

        loop {
            assert_eq!(cave[(y*width+x) as usize], b'.');

            if y >= height {
                break 'outer;
            }
            if cave[((y + 1) * width + x) as usize] == b'.' {
                y += 1;
                if y >= height-1 {
                    break 'outer;
                }
            } else if x > 0 && cave[((y + 1) * width + x - 1) as usize] == b'.' {
                y += 1;
                x -= 1;
                if x <= 0 {
                    break 'outer;
                }
            } else if x < width - 1 && cave[((y + 1) * width + x + 1) as usize] == b'.' {
                y += 1;
                x += 1;
                if x >= width-1 {
                    break 'outer;
                }
            } else {
                cave[(y * width + x) as usize] = b'o';
                break;
            }
        }

        println!("{count}");
        print_cave(&cave, width as usize);

        // if count == 24 {
        //     break;
        // }
    }
    count
}

fn print_cave(cave: &[u8], width: usize) {
    cave.chunks(width).for_each(|chunk| {
        println!("{}", from_utf8(chunk).unwrap());
    });
    println!();
}
