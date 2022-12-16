use adventofcode2022::read_lines;
use adventofcode2022::{regex, Error, Result};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Write};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Id(u16);

impl Id {
    fn try_new(s: &str) -> Result<Self> {
        let array: [u8; 2] = s.as_bytes().try_into().map_err(|_| Error::PatternMatch)?;
        if array[0] < b'A' || array[0] > b'Z' || array[1] < b'A' || array[1] > b'Z' {
            Err(Error::PatternMatch)
        } else {
            Ok(Self(((array[0] - b'A') as u16) * 26 + (array[1] - b'A') as u16))
        }
    }
    fn to_int(&self) -> usize {
        self.0 as _
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(((self.0 / 26) as u8 + b'A') as _)?;
        f.write_char(((self.0 % 26) as u8 + b'A') as _)?;
        Ok(())
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl FromStr for Id {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::try_new(s)
    }
}

#[derive(Debug, Clone)]
struct Valve {
    id: Id,
    flow_rate: u32,
    tunnels: Vec<Id>,
}

impl FromStr for Valve {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let pattern = regex!(
            "^Valve ([A-Z][A-Z]) has flow rate=([0-9]+); tunnels? leads? to valves? ([A-Z][A-Z](, [A-Z][A-Z])*)$"
        );
        let captures = pattern.captures(s).ok_or(Error::PatternMatch)?;
        let id = captures[1].parse()?;
        let flow_rate = captures[2].parse()?;
        let tunnels = captures[3]
            .split(", ")
            .map(|m| m.parse())
            .collect::<Result<Vec<Id>>>()?;
        Ok(Self { id, flow_rate, tunnels })
    }
}

fn solve_part1(
    valves_by_id: &[Option<Valve>],
    transitions: &[Vec<Id>],
    current_valve: Id,
    valve_states: Vec<u64>,
    minutes_left: usize,
    current_pressure: usize,
) {
}

pub fn main() -> Result<()> {
    let min_id = Id::try_new("AA").unwrap().to_int();
    let max_id = Id::try_new("ZZ").unwrap().to_int();

    dbg!(min_id, max_id);

    let mut valves = vec![None; max_id];

    read_lines("data/a16_example.txt")?
        .iter()
        .try_for_each(|line| -> Result<()> {
            let v = Valve::from_str(line)?;
            let idx = v.id.to_int();
            valves[idx] = Some(v);
            Ok(())
        })?;

    let mut transitions = vec![vec![]; max_id];

    valves
        .iter()
        .flatten()
        .for_each(|v| transitions[v.id.to_int()] = v.tunnels.clone());

    solve_part1(
        &valves,
        &transitions,
        Id::try_new("AA").unwrap(),
        vec![0; (max_id + 63) / 64],
        30,
        0,
    );

    Ok(())
}
