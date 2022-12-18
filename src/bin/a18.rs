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

    println!("part1: {}", solve_part1(&mut points, max));
    println!("part2: {}", solve_part2(&mut points));

    Ok(())
}

fn solve_part2(points: &mut Vec<Point>) -> usize {
    let mut cube = [[[false; 20]; 20]; 20];

    points.iter().for_each(|[x, y, z]| {
        cube[*x as usize][*y as usize][*z as usize] = true;
    });

    floodfill(&mut cube, 0, 0, 0);
    let air = cube.iter().flatten().flatten().filter(|b| !**b).count();
    dbg!(air);

    cube.iter().enumerate().for_each(|(x, slice)| {
        slice.iter().enumerate().for_each(|(y, line)| {
            line.iter().enumerate().for_each(|(z, b)| {
                if !*b {
                    points.push([x as u32, y as u32, z as u32]);
                }
            })
        })
    });

    solve_part1(points, [20, 20, 20])
}

fn floodfill<const S: usize>(cube: &mut [[[bool; S]; S]; S], x: usize, y: usize, z: usize) {
    if cube[x][y][z] {
        return;
    }
    cube[x][y][z] = true;
    if x > 0 {
        floodfill(cube, x - 1, y, z);
    }
    if x < S - 1 {
        floodfill(cube, x + 1, y, z);
    }
    if y > 0 {
        floodfill(cube, x, y - 1, z);
    }
    if y < S - 1 {
        floodfill(cube, x, y + 1, z);
    }
    if z > 0 {
        floodfill(cube, x, y, z - 1);
    }
    if z < S - 1 {
        floodfill(cube, x, y, z + 1)
    }
}

fn solve_part1(points: &mut Vec<Point>, max: [u32; 3]) -> usize {
    let c1 = count_sides(points, max, 0, 1, 2);
    let c2 = count_sides(points, max, 0, 2, 1);
    let c3 = count_sides(points, max, 1, 2, 0);

    c1 + c2 + c3
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
                                (None, _) => 1,
                                (Some(old_x), x) if x == old_x + 1 => 0,
                                _ => 2,
                            };

                            (Some(z), count + inc)
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
