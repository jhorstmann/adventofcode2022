use adventofcode2022::read_lines;
use adventofcode2022::{regex, Error, Result};
use std::str::{from_utf8, FromStr};

#[derive(Debug, Clone)]
struct Sensor {
    position: (i64, i64),
    closest_beacon: (i64, i64),
}

impl FromStr for Sensor {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let pattern = regex!("^Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)");
        let captures = pattern.captures(s).ok_or(Error::PatternMatch)?;
        let sx = captures.get(1).unwrap().as_str().parse()?;
        let sy = captures.get(2).unwrap().as_str().parse()?;
        let bx = captures.get(3).unwrap().as_str().parse()?;
        let by = captures.get(4).unwrap().as_str().parse()?;
        Ok(Sensor {
            position: (sx, sy),
            closest_beacon: (bx, by),
        })
    }
}

impl Sensor {
    fn min_range(&self) -> u64 {
        self.position.0.abs_diff(self.closest_beacon.0) + self.position.1.abs_diff(self.closest_beacon.1)
    }
}

pub fn main() -> Result<()> {
    let sensors = read_lines("data/a15.txt")?
        .iter()
        .map(|line| Sensor::from_str(line))
        .collect::<Result<Vec<Sensor>>>()?;

    // dbg!(&sensors);

    // for y in -2..20 {
    // solve_part1(&[sensors[6].clone()], y);
    // solve_part1(&sensors, y);
    // }
    println!();
    let part1 = solve_part1(&sensors, 2000000);
    println!("part1: {part1}");

    Ok(())
}

fn solve_part1(sensors: &[Sensor], ty: i64) -> usize {
    let (min, max) = sensors.iter().fold((i64::MAX, i64::MIN), |mut acc, s| {
        acc.0 = acc.0.min(s.position.0 - s.min_range() as i64);
        acc.1 = acc.0.max(s.position.0 + s.min_range() as i64);
        acc
    });

    // dbg!(min, max);

    let mut line = vec![b'.'; (max) as usize * 2];
    line.iter_mut().enumerate().for_each(|(i, b)| {
        let tx = min + (i as i64);
        let is_beacon = sensors.iter().any(|s| s.closest_beacon == (tx, ty));
        let is_sensor = sensors.iter().any(|s| s.position == (tx, ty));
        let in_range = sensors.iter().any(|s| {
            let dist = s.position.0.abs_diff(tx) + s.position.1.abs_diff(ty);
            dist <= s.min_range()
        });
        if is_beacon {
            *b = b'B';
        } else if is_sensor {
            *b = b'S';
        } else if in_range {
            *b = b'#';
        }
    });

    let count = line.iter().filter(|b| **b == b'#').count();
    println!("{:2} {} ({:3})", ty, from_utf8(&line).unwrap(), count);

    count
}
