use adventofcode2022::Result;
use adventofcode2022::{read_lines, regex, Error};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        fn parse_positive(s: &str) -> Result<usize> {
            s.parse::<usize>()?.checked_sub(1).ok_or(Error::PatternMatch)
        }

        let pattern = regex!("^move (\\d+) from (\\d+) to (\\d+)$");
        let captures = pattern.captures(s).ok_or(Error::PatternMatch)?;
        let amount = parse_positive(captures.get(1).unwrap().as_str())? + 1;
        let from = parse_positive(captures.get(2).unwrap().as_str())?;
        let to = parse_positive(captures.get(3).unwrap().as_str())?;
        Ok(Self { amount, from, to })
    }
}

type Stacks = Vec<Vec<u8>>;

fn main() -> Result<()> {
    let lines = read_lines("data/a5.txt")?;

    let mut split = lines.split(String::is_empty);
    let mut picture = split.next().unwrap().to_vec();

    let stack_numbers = picture
        .pop()
        .ok_or(Error::EmptyIterator)?
        .trim()
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>())
        .collect::<std::result::Result<Vec<usize>, _>>()?;

    let mut stacks = vec![Vec::default(); stack_numbers.len()];

    picture.iter().for_each(|s| {
        s.as_bytes().chunks(4).enumerate().for_each(|(i, c)| {
            if c[0] == b'[' {
                stacks[i].push(c[1]);
            }
        });
    });

    stacks.iter_mut().for_each(|stack| stack.reverse());
    let initial_stacks = stacks.clone();

    let instructions = split.next().unwrap();
    let instructions = instructions
        .iter()
        .map(|insn| Instruction::from_str(insn))
        .collect::<Result<Vec<Instruction>>>()?;

    instructions.iter().for_each(|insn| {
        for _ in 0..insn.amount {
            let x = stacks[insn.from].pop().unwrap();
            stacks[insn.to].push(x);
        }
    });

    let part1 = String::from_utf8(
        stacks
            .iter()
            .map(|stack| stack.last().copied().unwrap_or(b' '))
            .collect(),
    )?;

    println!("part1: {}", part1);

    stacks = initial_stacks;

    instructions.iter().for_each(|insn| {
        let mut tmp = vec![];
        for _ in 0..insn.amount {
            let x = stacks[insn.from].pop().unwrap();
            tmp.push(x);
        }
        tmp.reverse();
        stacks[insn.to].extend_from_slice(&tmp);
    });

    let part2 = String::from_utf8(
        stacks
            .iter()
            .map(|stack| stack.last().copied().unwrap_or(b' '))
            .collect(),
    )?;

    println!("part2: {}", part2);

    Ok(())
}
