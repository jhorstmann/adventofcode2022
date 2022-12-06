use adventofcode2022::{local_regex, read_lines, Error, Result};
use std::num::NonZeroUsize;
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
            Ok(s.parse::<NonZeroUsize>()?.get().wrapping_sub(1))
        }

        let pattern = local_regex!("^move (\\d+) from (\\d+) to (\\d+)$");
        let captures = pattern.captures(s).ok_or(Error::PatternMatch)?;
        let amount = parse_positive(captures.get(1).unwrap().as_str())? + 1;
        let from = parse_positive(captures.get(2).unwrap().as_str())?;
        let to = parse_positive(captures.get(3).unwrap().as_str())?;
        Ok(Self { amount, from, to })
    }
}

type Stacks = Vec<Vec<u8>>;

fn part1(stacks: &mut Stacks, instructions: &[Instruction]) {
    instructions.iter().for_each(|insn| {
        for _ in 0..insn.amount {
            let x = stacks[insn.from].pop().unwrap();
            stacks[insn.to].push(x);
        }
    });
}

fn part2(stacks: &mut Stacks, instructions: &[Instruction]) {
    let mut tmp = vec![];
    instructions.iter().for_each(|insn| {
        tmp.clear();
        for _ in 0..insn.amount {
            let x = stacks[insn.from].pop().unwrap();
            tmp.push(x);
        }
        tmp.reverse();
        stacks[insn.to].extend_from_slice(&tmp);
    });
}

fn stack_top_to_string(stacks: &Stacks) -> String {
    stacks
        .iter()
        .map(|stack| {
            stack
                .last()
                .copied()
                .map(char::from)
                .unwrap_or(char::REPLACEMENT_CHARACTER)
        })
        .collect()
}

fn main() -> Result<()> {
    let lines = read_lines("data/a5.txt")?;

    let mut split = lines.split(String::is_empty);
    let mut picture = split.next().unwrap().to_vec();

    let stack_numbers = picture
        .pop()
        .ok_or(Error::EmptyIterator)?
        .trim()
        .split_ascii_whitespace()
        .map(|n| n.parse::<NonZeroUsize>())
        .collect::<std::result::Result<Vec<NonZeroUsize>, _>>()?;

    let mut stacks = vec![Vec::default(); stack_numbers.len()];

    picture.iter().for_each(|s| {
        s.as_bytes().chunks(4).enumerate().for_each(|(i, c)| {
            if c[0] == b'[' {
                stacks[i].push(c[1]);
            }
        });
    });

    stacks.iter_mut().for_each(|stack| stack.reverse());

    let instructions = split.next().unwrap();
    let instructions = instructions
        .iter()
        .map(|insn| Instruction::from_str(insn))
        .collect::<Result<Vec<Instruction>>>()?;

    {
        let mut stacks = stacks.clone();
        part1(&mut stacks, &instructions);
        println!("part1: {}", stack_top_to_string(&stacks));
    }

    {
        let mut stacks = stacks.clone();
        part2(&mut stacks, &instructions);
        println!("part2: {}", stack_top_to_string(&stacks));
    }

    Ok(())
}
