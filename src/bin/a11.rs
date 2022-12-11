use adventofcode2022::read_lines;
use adventofcode2022::{Error, Result};
use std::cmp::Reverse;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Square,
    Mul(usize),
    Add(usize),
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s == "* old" {
            return Ok(Operation::Square);
        }
        if let Some(n) = s.strip_prefix('*') {
            return Ok(Operation::Mul(n.trim_start().parse()?));
        }
        if let Some(n) = s.strip_prefix('+') {
            return Ok(Operation::Add(n.trim_start().parse()?));
        }
        Err(Error::PatternMatch)
    }
}

impl Operation {
    fn apply(&self, x: usize) -> usize {
        match self {
            Operation::Square => x * x,
            Operation::Mul(y) => x * y,
            Operation::Add(y) => x + y,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    condition: usize,
    then_target: usize,
    else_target: usize,
    inspections: usize,
}

impl Monkey {
    fn try_from_chunk(i: usize, lines: &[String]) -> Result<Monkey> {
        let mut lines = lines.iter();
        let n: usize = lines
            .next()
            .ok_or(Error::EmptyIterator)?
            .strip_prefix("Monkey ")
            .ok_or(Error::PatternMatch)?
            .strip_suffix(':')
            .ok_or(Error::PatternMatch)?
            .parse()?;
        if n != i {
            return Err(Error::General(format!("Unexpected monkey {} at {}", n, i)));
        }

        let items = lines
            .next()
            .ok_or(Error::EmptyIterator)?
            .strip_prefix("  Starting items: ")
            .ok_or(Error::PatternMatch)?
            .split(", ")
            .map(|item| Ok(item.parse()?))
            .collect::<Result<Vec<usize>>>()?;
        let operation = lines
            .next()
            .ok_or(Error::EmptyIterator)?
            .strip_prefix("  Operation: new = old ")
            .ok_or(Error::PatternMatch)?;
        let operation = Operation::from_str(operation)?;
        let condition = lines
            .next()
            .ok_or(Error::EmptyIterator)?
            .strip_prefix("  Test: divisible by ")
            .ok_or(Error::PatternMatch)?;
        let condition = condition.parse()?;
        let then_target = lines
            .next()
            .ok_or(Error::EmptyIterator)?
            .strip_prefix("    If true: throw to monkey ")
            .ok_or(Error::PatternMatch)?
            .parse()?;
        let else_target = lines
            .next()
            .ok_or(Error::EmptyIterator)?
            .strip_prefix("    If false: throw to monkey ")
            .ok_or(Error::PatternMatch)?
            .parse()?;

        let inspections = 0;
        Ok(Monkey {
            items,
            operation,
            condition,
            then_target,
            else_target,
            inspections,
        })
    }
}

pub fn main() -> Result<()> {
    let monkeys = read_lines("data/a11.txt")?
        .split(|line| line.is_empty())
        .enumerate()
        .map(|(i, chunk)| Monkey::try_from_chunk(i, chunk))
        .collect::<Result<Vec<Monkey>>>()?;

    println!("part1: {}", solve::<true>(monkeys.clone(), 20));
    println!("part2: {}", solve::<false>(monkeys, 10_000));

    Ok(())
}

fn solve<const DIV: bool>(mut monkeys: Vec<Monkey>, rounds: usize) -> usize {
    let gcd: usize = monkeys.iter().map(|m| m.condition).product();
    for _round in 0..rounds {
        for m in 0..monkeys.len() {
            let op = monkeys[m].operation;
            let cond = monkeys[m].condition;
            let then_target = monkeys[m].then_target;
            let else_target = monkeys[m].else_target;
            for i in std::mem::take(&mut monkeys[m].items) {
                let mut level = op.apply(i);
                if DIV {
                    level /= 3;
                } else {
                    level %= gcd;
                }

                let target = [else_target, then_target][(level % cond == 0) as usize];
                monkeys[target].items.push(level);
                monkeys[m].inspections += 1;
            }
        }
    }

    monkeys.sort_by_key(|m| Reverse(m.inspections));

    monkeys[0].inspections * monkeys[1].inspections
}
