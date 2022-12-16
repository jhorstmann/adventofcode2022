use adventofcode2022::read_lines;
use adventofcode2022::{regex, Error, Result};
use std::collections::{BinaryHeap, HashSet};
use std::fmt::{Debug, Display, Formatter, Write};
use std::hash::Hash;
use std::str::FromStr;

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
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

    fn as_int(&self) -> usize {
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
    flow_rate: u16,
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

#[inline(always)]
fn bittest(bitmap: &[u64], idx: usize) -> bool {
    bitmap[idx / 64] & (1 << (idx % 64)) != 0
}

#[inline(always)]
fn bitset(bitmap: &mut [u64], idx: usize) {
    bitmap[idx / 64] |= 1 << (idx % 64)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
struct State {
    total_pressure: u16,
    pressure_increment: u16,
    minutes_left: u16,
    current_valve_id: Id,
    active_valves_bitmap: [u64; 11],
}

fn solve_part1(valves_by_id: &[Option<Valve>], transitions: &[Vec<Id>]) -> usize {
    let mut queue = BinaryHeap::new();
    let initial_state = State {
        minutes_left: 30,
        // active_valves_bitmap: vec![0_u64; (valves_by_id.len() + 63) / 64],
        ..Default::default()
    };
    queue.push(initial_state);

    let mut visited = HashSet::with_capacity(16 * 1024);

    let mut max = 0;

    while let Some(state) = queue.pop() {
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state.clone());

        let new_max = state.total_pressure + state.pressure_increment * state.minutes_left;
        if new_max > max {
            // dbg!(max, state.minutes_left, state.pressure_increment, queue.len());
            max = new_max;
        }
        if state.minutes_left > 0 {
            let current_valve = valves_by_id[state.current_valve_id.as_int()].as_ref().unwrap();

            if current_valve.flow_rate > 0 && !bittest(&state.active_valves_bitmap, current_valve.id.as_int()) {
                let mut new_state = state.clone();
                new_state.minutes_left -= 1;
                bitset(&mut new_state.active_valves_bitmap, current_valve.id.as_int());
                new_state.total_pressure += state.pressure_increment;
                new_state.pressure_increment += current_valve.flow_rate;
                queue.push(new_state);
            }

            for next_id in &transitions[current_valve.id.as_int()] {
                let mut new_state = state.clone();
                new_state.total_pressure += state.pressure_increment;
                new_state.minutes_left -= 1;
                new_state.current_valve_id = *next_id;
                queue.push(new_state);
            }
        }
    }

    max.into()
}

pub fn main() -> Result<()> {
    let max_id = Id::try_new("ZZ").unwrap().as_int();

    let valves = read_lines("data/a16.txt")?
        .iter()
        .map(|line| Valve::from_str(line))
        .collect::<Result<Vec<_>>>()?;

    let mut valves_by_id = vec![None; max_id];
    valves.iter().cloned().try_for_each(|v| -> Result<()> {
        let idx = v.id.as_int();
        valves_by_id[idx] = Some(v);
        Ok(())
    })?;

    let mut transitions = vec![vec![]; max_id];

    valves
        .iter()
        .for_each(|v| transitions[v.id.as_int()] = v.tunnels.clone());

    println!("part1: {}", solve_part1(&valves_by_id, &transitions));

    Ok(())
}
