use adventofcode2022::read_lines;
use adventofcode2022::Result;
use std::cmp::Ordering;

#[repr(u8)]
#[derive(Eq, PartialEq, Clone, Copy)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<u8> for RPS {
    fn from(character: u8) -> Self {
        match character {
            b'A' | b'X' => Self::Rock,
            b'B' | b'Y' => Self::Paper,
            b'C' | b'Z' => Self::Scissors,
            _ => panic!(),
        }
    }
}

impl PartialOrd<Self> for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (s, o) if s == o => Some(Ordering::Equal),
            (Self::Rock, Self::Scissors) => Some(Ordering::Greater),
            (Self::Scissors, Self::Paper) => Some(Ordering::Greater),
            (Self::Paper, Self::Rock) => Some(Ordering::Greater),
            _ => Some(Ordering::Less),
        }
    }
}

impl RPS {
    fn value(&self) -> u64 {
        *self as u64
    }

    fn choose(&self, ordering: Ordering) -> Self {
        match (ordering, *self) {
            (Ordering::Equal, _) => *self,
            (Ordering::Greater, Self::Rock) => Self::Paper,
            (Ordering::Greater, Self::Paper) => Self::Scissors,
            (Ordering::Greater, Self::Scissors) => Self::Rock,
            (Ordering::Less, Self::Rock) => Self::Scissors,
            (Ordering::Less, Self::Paper) => Self::Rock,
            (Ordering::Less, Self::Scissors) => Self::Paper,
        }
    }
}

fn score(p1: RPS, p2: RPS) -> u64 {
    p1.value()
        + match p1.partial_cmp(&p2).unwrap() {
            Ordering::Equal => 3,
            Ordering::Greater => 6,
            Ordering::Less => 0,
        }
}

fn main() -> Result<()> {
    let strategy = read_lines("data/a2.txt")?;

    let sum = strategy
        .iter()
        .map(|line| {
            let mut split = line.split(" ");
            let opponent = RPS::from(split.next().unwrap().as_bytes()[0]);
            let you = RPS::from(split.next().unwrap().as_bytes()[0]);
            score(you, opponent)
        })
        .sum::<u64>();

    println!("part1: {sum}");

    let sum2 = strategy
        .iter()
        .map(|line| {
            let mut split = line.split(" ");
            let opponent = RPS::from(split.next().unwrap().as_bytes()[0]);
            let ordering = match split.next().unwrap().as_bytes()[0] {
                b'X' => Ordering::Less,
                b'Y' => Ordering::Equal,
                b'Z' => Ordering::Greater,
                _ => panic!(),
            };
            let you = opponent.choose(ordering);
            score(you, opponent)
        })
        .sum::<u64>();

    println!("part1: {sum2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::RPS;

    #[test]
    fn test_rps() {
        assert!(RPS::Rock == RPS::Rock);
        assert!(RPS::Rock > RPS::Scissors);
        assert!(RPS::Rock < RPS::Paper);
    }
}
