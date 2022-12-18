use adventofcode2022::read_lines;
use adventofcode2022::{Error, Result};

type Point = [u32; 3];

pub fn main() -> Result<()> {
    let mut points = read_lines("data/a18.txt")?
        .iter()
        .map(|line| {
            let mut split = line.splitn(3, ',');
            let x = split.next().ok_or(Error::EmptyIterator)?.parse()?;
            let y = split.next().ok_or(Error::EmptyIterator)?.parse()?;
            let z = split.next().ok_or(Error::EmptyIterator)?.parse()?;
            Ok([x, y, z])
        })
        .collect::<Result<Vec<Point>>>()?;

    let max = points.iter().fold([0_u32; 3], |acc, p| {
        [acc[0].max(p[0]), acc[1].max(p[1]), acc[2].max(p[2])]
    });

    let c1 = count_sides(&mut points, max, 0, 1, 2);
    let c2 = count_sides(&mut points, max, 0, 2, 1);
    let c3 = count_sides(&mut points, max, 1, 2, 0);

    let part1 = c1 + c2 + c3;
    println!("part1: {part1}");

    Ok(())
}

fn count_sides(points: &mut Vec<Point>, max: [u32; 3], dir_x: usize, dir_y: usize, dir_z: usize) -> usize {
    points.sort_by_key(|p| p[dir_z]);

    (0..max[dir_x] + 1)
        .map(|x| {
            (0..max[dir_y] + 1)
                .map(|y| {
                    let (last_z, count) = points.iter().filter(|p| p[dir_x] == x && p[dir_y] == y).fold(
                        (None, 0_usize),
                        |(last_z, count), p| {
                            let z = p[dir_z];
                            let inc = match (last_z, z) {
                                (None, x) => 1,
                                (Some(old_x), x) if x == old_x + 1 => 0,
                                _ => 2,
                            };

                            (Some(p[dir_z]), count + inc)
                        },
                    );
                    if last_z.is_some() {
                        count + 1
                    } else {
                        count
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}
