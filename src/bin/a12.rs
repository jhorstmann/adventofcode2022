use adventofcode2022::read_lines;
use adventofcode2022::Error;
use adventofcode2022::Result;

pub fn main() -> Result<()> {
    let lines = read_lines("data/a12.txt")?;
    let height = lines.len();
    let mut grid = lines
        .into_iter()
        .flat_map(|line| line.into_bytes())
        .collect::<Vec<u8>>();
    assert_eq!(grid.len() % height, 0);
    let width = grid.len() / height;

    let mut start = (0, 0);
    let mut target = (0, 0);

    grid.iter_mut().enumerate().for_each(|(i, c)| {
        let x = (i % width) as isize;
        let y = (i / width) as isize;
        if *c == b'S' {
            start = (x, y);
            *c = 0;
        } else if *c == b'E' {
            target = (x, y);
            *c = b'z' - b'a'
        } else {
            *c -= b'a'
        }
    });

    let part1 = solve_part1(&grid, width, start, target);
    println!("part1: {part1}");

    let mut min = usize::MAX;
    grid.iter().enumerate().for_each(|(i, c)| {
        if *c == 0 {
            let x = (i % width) as isize;
            let y = (i / width) as isize;
            let dist = solve_part1(&grid, width, (x, y), target);
            if dist < min {
                min = dist;
            }
        }
    });

    println!("part2: {min}");

    Ok(())
}

fn solve_part1(grid: &[u8], width: usize, current: (isize, isize), target: (isize, isize)) -> usize {
    assert_eq!(grid.len() % width, 0);
    let height = (grid.len() / width) as isize;
    let width = width as isize;
    let (tx, ty) = target;
    let mut steps = vec![usize::MAX; grid.len()];
    let mut queue = vec![(current, 0_usize)];
    while let Some(((x, y), s)) = queue.pop() {
        if s >= steps[(y * width + x) as usize] {
            continue;
        }

        steps[(y * width + x) as usize] = s;
        if (x, y) == (tx, ty) {
            continue;
        }

        let z = grid[(y * width + x) as usize] as isize;

        for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && nx < width && ny >= 0 && ny < height {
                if (grid[(ny * width + nx) as usize] as isize) <= z + 1 {
                    queue.push(((nx, ny), s + 1));
                }
            }
        }
    }
    steps[(ty * width + tx) as usize]
}
