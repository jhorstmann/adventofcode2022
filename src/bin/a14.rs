use adventofcode2022::{read_lines, Result};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut, Range};
use std::str::from_utf8;

struct Grid<T, S: AsRef<[T]>> {
    data: S,
    width: usize,
    height: usize,
    phantom: PhantomData<T>,
}

impl<T, S: AsRef<[T]>> Grid<T, S> {
    fn new(data: S, width: usize) -> Self {
        assert!(width <= isize::MAX as usize);
        assert!(data.as_ref().len() % width == 0);
        let height = data.as_ref().len() / width;
        assert!(height <= isize::MAX as usize);
        Self {
            data,
            width,
            height,
            phantom: PhantomData::default(),
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<&T> {
        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
            None
        } else {
            Some(&self.data.as_ref()[(y as usize) * self.width + (x as usize)])
        }
    }
}

impl<T, S: AsRef<[T]>> AsRef<[T]> for Grid<T, S> {
    fn as_ref(&self) -> &[T] {
        self.data.as_ref()
    }
}

impl<T, S: AsRef<[T]> + AsMut<[T]>> Grid<T, S> {
    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
            None
        } else {
            Some(&mut self.data.as_mut()[(y as usize) * self.width + (x as usize)])
        }
    }
}

impl<T, S: AsRef<[T]>> Index<(isize, isize)> for Grid<T, S> {
    type Output = T;

    fn index(&self, (x, y): (isize, isize)) -> &T {
        self.get(x, y).expect("index out of bounds")
    }
}

impl<T, S: AsRef<[T]> + AsMut<[T]>> IndexMut<(isize, isize)> for Grid<T, S> {
    fn index_mut(&mut self, (x, y): (isize, isize)) -> &mut T {
        self.get_mut(x, y).expect("index out of bounds")
    }
}

impl<T, S: AsRef<[T]> + Clone> Clone for Grid<T, S> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            width: self.width,
            height: self.height,
            phantom: self.phantom,
        }
    }
}

pub fn main() -> Result<()> {
    let lines = read_lines("data/a14.txt")?;
    let mut paths: Vec<Vec<(isize, isize)>> = lines
        .iter()
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

    let (cave, min_x, max_x, min_y, max_y) = create_grid(&paths);
    // println!("part1: {}", part1(cave.clone(), 500 - min_x, 0));

    paths.push(vec![
        (min_x - (max_y - min_y), max_y + 2),
        (max_x + (max_y - min_y), max_y + 2),
    ]);

    let (cave, min_x, max_x, min_y, max_y) = create_grid(&paths);
    println!("part2: {}", part1(cave.clone(), 500 - min_x, 0));

    Ok(())
}

fn create_grid(paths: &[Vec<(isize, isize)>]) -> (Grid<u8, Vec<u8>>, isize, isize, isize, isize) {
    let (min_x, max_x, min_y, max_y) = paths.iter().flatten().copied().chain(std::iter::once((500, 0))).fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |mut acc, p| {
            acc.0 = acc.0.min(p.0);
            acc.1 = acc.1.max(p.0);
            acc.2 = acc.2.min(p.1);
            acc.3 = acc.3.max(p.1);
            acc
        },
    );

    dbg!(min_x, max_x, min_y, max_y);

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let mut cave = Grid::new(vec![b'.'; (width * height)], width);

    paths.iter().for_each(|path| {
        path.windows(2).for_each(|segment| {
            let (sx, sy) = segment[0];
            let (tx, ty) = segment[1];

            let dx = (tx - sx).signum();
            let dy = (ty - sy).signum();

            let (mut x, mut y) = (sx, sy);
            loop {
                cave[(x - min_x, y - min_y)] = b'#';
                if x == tx && y == ty {
                    break;
                }
                x += dx;
                y += dy;
            }
        })
    });
    (cave, min_x, max_x, min_y, max_y)
}

fn part1(mut cave: Grid<u8, Vec<u8>>, sx: isize, sy: isize) -> usize {
    let height = cave.height as isize;
    let width = cave.width as isize;
    let mut count = 0_usize;
    'outer: loop {
        let mut x = sx as isize;
        let mut y = sy as isize;

        if cave[(x, y)] != b'.' {
            print_cave(cave.as_ref(), width as usize);
            break;
        }

        loop {
            // assert_eq!(cave[(x, y)], b'.');

            if y >= height {
                break 'outer;
            }
            if cave[(x, y + 1)] == b'.' {
                y += 1;
                if y >= height - 1 {
                    break 'outer;
                }
            } else if x > 0 && cave[(x - 1, y + 1)] == b'.' {
                y += 1;
                x -= 1;
                if x <= 0 {
                    break 'outer;
                }
            } else if x < width - 1 && cave[(x + 1, y + 1)] == b'.' {
                y += 1;
                x += 1;
                if x >= width - 1 {
                    break 'outer;
                }
            } else {
                cave[(x, y)] = b'o';
                break;
            }
        }
        count += 1;

        // println!("{count}");
        // print_cave(cave.as_ref(), width as usize);

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
