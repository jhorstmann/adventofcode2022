use adventofcode2022::read_lines;
use adventofcode2022::{regex, Error, Result};
use std::ops::{Range, RangeInclusive};
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

    fn boundary_points(&self) -> impl Iterator<Item = (i64, i64)> {
        let d = self.min_range();
        boundary_points(self.position.0, self.position.1, d)
    }

    fn in_range(&self, x: i64, y: i64) -> bool {
        let dist = self.position.0.abs_diff(x) + self.position.1.abs_diff(y);
        dist <= self.min_range()
    }
}

fn boundary_points(x: i64, y: i64, distance: u64) -> impl Iterator<Item = (i64, i64)> {
    (0..=distance).flat_map(move |d| {
        [
            (x - (distance - d) as i64, y - d as i64),
            (x - (distance - d) as i64, y + d as i64),
            (x + (distance - d) as i64, y - d as i64),
            (x + (distance - d) as i64, y + d as i64),
        ]
    })
}

pub fn main() -> Result<()> {
    let sensors = read_lines("data/a15.txt")?
        .iter()
        .map(|line| Sensor::from_str(line))
        .collect::<Result<Vec<Sensor>>>()?;

    // dbg!(&sensors);

    let part1 = solve_part1(&sensors, 10);
    println!("part1: {part1}");

    // dbg!(boundary_points(0, 0, 2).collect::<Vec<_>>());

    let part2 = solve_part2(&sensors, 0..=4_000_000);
    println!("part2: {part2}");

    Ok(())
}

fn solve_part1(sensors: &[Sensor], ty: i64) -> usize {
    let (min, max) = sensors.iter().fold((i64::MAX, i64::MIN), |mut acc, s| {
        acc.0 = acc.0.min(s.position.0 - s.min_range() as i64);
        acc.1 = acc.0.max(s.position.0 + s.min_range() as i64);
        acc
    });

    // dbg!(min, max);

    let mut line = vec![b'.'; (max - min) as usize * 2];
    line.iter_mut().enumerate().for_each(|(i, b)| {
        let tx = min + (i as i64);
        let is_beacon = sensors.iter().any(|s| s.closest_beacon == (tx, ty));
        let is_sensor = sensors.iter().any(|s| s.position == (tx, ty));
        let in_range = sensors.iter().any(|s| s.in_range(tx, ty));
        if is_beacon {
            *b = b'B';
        } else if is_sensor {
            *b = b'S';
        } else if in_range {
            *b = b'#';
        }
    });

    let count = line.iter().filter(|b| **b == b'#').count();
    // println!("{:2} {} ({:3})", ty, from_utf8(&line).unwrap(), count);

    count
}

fn solve_part2(sensors: &[Sensor], range: RangeInclusive<i64>) -> i64 {
    let mut candidates = sensors
        .iter()
        .flat_map(|s| boundary_points(s.position.0, s.position.1, s.min_range() + 1))
        .filter(|point: &(i64, i64)| range.contains(&point.0) && range.contains(&point.1))
        .filter(|point| !sensors.iter().any(|s| s.in_range(point.0, point.1)))
        .collect::<Vec<(i64, i64)>>();

    // dbg!(&candidates);

    candidates.first().map(|(x, y)| x * 4_000_000 + y).unwrap_or_default()
}
