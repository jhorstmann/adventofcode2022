use adventofcode2022::read_lines;
use adventofcode2022::Error;
use adventofcode2022::Result;
use std::str::from_utf8;

struct Rock {
    shape: u16,
    width: u8,
    height: u8,
}

impl Rock {
    const fn new(shape: u16, width: usize, height: usize) -> Self {
        assert!(width <= 4);
        assert!(height <= 4);
        Self {
            shape,
            width: width as _,
            height: height as _,
        }
    }

    fn row(&self, i: usize) -> u8 {
        ((self.shape >> (i * 4)) & 0b1111) as _
    }

    fn valid_position(&self, chamber: &[u8], width: usize, x: isize, y: isize) -> bool {
        if x < 0 || (x as usize) + (self.width as usize) > width {
            return false;
        }
        (0..self.height as usize).all(|yy| chamber[y as usize + yy] & (self.row(yy) << x) == 0)
    }
}

const SHAPE_LINE: Rock = Rock::new(0b0000_0000_0000_1111, 4, 1);
const SHAPE_PLUS: Rock = Rock::new(0b0000_0010_0111_0010, 3, 3);
const SHAPE_EDGE: Rock = Rock::new(0b0100_0100_0100_0111, 3, 3);
const SHAPE_CAPI: Rock = Rock::new(0b0001_0001_0001_0001, 1, 4);
const SHAPE_BLCK: Rock = Rock::new(0b0000_0000_0011_0011, 2, 2);

pub fn main() -> Result<()> {
    let lines = read_lines("data/a17_example.txt")?;
    let winds = lines.first().ok_or(Error::EmptyIterator)?;

    let mut rocks_iter = [SHAPE_LINE, SHAPE_PLUS, SHAPE_EDGE, SHAPE_CAPI, SHAPE_BLCK]
        .iter()
        .cycle();
    let mut winds_iter = winds.as_bytes().iter().copied().cycle();

    let mut chamber = Vec::<u8>::new();
    let mut counter = 0;
    loop {
        while let Some(top) = chamber.last() {
            if *top == 0 {
                chamber.pop();
            } else {
                break;
            }
        }
        // dbg!(counter, chamber.len());
        // print_chamber(&chamber, None);

        if counter == 2022 {
            break;
        }
        let next_rock = rocks_iter.next().unwrap();
        counter += 1;

        chamber.extend((0..3).map(|_| 0));
        chamber.extend((0..next_rock.height).map(|_| 0));

        let mut x = 2_usize;
        let mut y = chamber.len() - next_rock.height as usize;

        let mut r = 0;
        loop {
            if r % 2 == 0 {
                // print_chamber(&chamber, Some((&next_rock, x, y)));
                let next_wind = winds_iter.next().unwrap();
                if next_wind == b'<' {
                    if next_rock.valid_position(&chamber, 7, x as isize - 1, y as isize) {
                        x -= 1;
                    }
                } else if next_wind == b'>' {
                    if next_rock.valid_position(&chamber, 7, x as isize + 1, y as isize) {
                        x += 1;
                    }
                }
                r += 1;
                // print_chamber(&chamber, Some((&next_rock, x, y)));
            } else {
                // print_chamber(&chamber, Some((&next_rock, x, y)));
                if y > 0 && next_rock.valid_position(&chamber, 7, x as isize, (y as isize - 1)) {
                    y -= 1;
                    r += 1;
                } else {
                    for yy in 0..next_rock.height as usize {
                        chamber[(y + yy)] |= next_rock.row(yy) << x;
                    }
                    // print_chamber(&chamber, None);
                    break;
                }
            }
        }

        // print_chamber(&chamber, Some((next_rock, x, y)));

        // print_chamber(&chamber, None);

        // if chamber.len() > 20 {
        //     break;
        // }
    }

    println!("part1: {}", chamber.len());

    Ok(())
}

fn print_chamber(chamber: &[u8], rock: Option<(&Rock, usize, usize)>) {
    let mut c = chamber
        .iter()
        .copied()
        .flat_map(move |row| (0..7).map(move |x| if row & (1 << x) != 0 { b'#' } else { b'.' }))
        .collect::<Vec<u8>>();

    if let Some((rock, x, y)) = rock {
        for yy in 0..rock.height as usize {
            let row = rock.row(yy);
            for xx in 0..rock.width as usize {
                if row & (1 << (xx)) != 0 {
                    if let Some(p) = c.get_mut((y + yy) * 7 + x + xx) {
                        *p = b'@';
                    }
                }
            }
        }
    }

    c.chunks(7).rev().for_each(|row| {
        println!("|{}|", from_utf8(row).unwrap());
    });
    println!("+-------+");
    println!();
}
