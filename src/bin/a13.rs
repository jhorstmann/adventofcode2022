use adventofcode2022::read_lines;
use adventofcode2022::Error;
use adventofcode2022::Result;
use serde_json::Value;
use std::cmp::Ordering;

#[derive(Debug)]
enum Packet {
    Integer(i64),
    List(Vec<Packet>),
}

impl TryFrom<serde_json::Value> for Packet {
    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        match value {
            Value::Number(n) => Ok(Packet::Integer(
                n.as_i64()
                    .ok_or(Error::General(format!("Unexpected non-integer number")))?,
            )),
            Value::Array(a) => Ok(Packet::List(
                a.into_iter().map(Packet::try_from).collect::<Result<Vec<_>>>()?,
            )),
            _ => Err(Error::General(format!("Unexpected json value {:?}", value))),
        }
    }
}

impl Packet {
    fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(i1), Packet::Integer(i2)) => i1.cmp(i2),
            (Packet::List(l1), Packet::List(l2)) => l1.cmp(l2),
            (Packet::Integer(i1), Packet::List(l2)) => vec![Packet::Integer(*i1)].cmp(l2),
            (Packet::List(l1), Packet::Integer(i2)) => l1.cmp(&vec![Packet::Integer(*i2)]),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == Ordering::Equal
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Eq for Packet {}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

pub fn main() -> Result<()> {
    let lines = read_lines("data/a13.txt")?;
    let pairs = lines
        .split(|line| line.is_empty())
        .map(|pair| {
            let left: serde_json::Value = pair[0].parse()?;
            let right: serde_json::Value = pair[1].parse()?;
            let left = Packet::try_from(left)?;
            let right = Packet::try_from(right)?;
            Ok::<_, Error>((left, right))
        })
        .collect::<Result<Vec<(Packet, Packet)>>>()?;

    // dbg!(&pairs);

    // for (left, right) in pairs.iter() {
    //     dbg!(left.compare(&right) == Ordering::Less);
    //
    // }

    let part1: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left.compare(right) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum();

    println!("part1: {part1}");

    Ok(())
}
