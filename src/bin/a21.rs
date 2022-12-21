use adventofcode2022::read_lines;
use adventofcode2022::{Error, Result};
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone)]
enum Monkey {
    Add(u32, u32),
    Sub(u32, u32),
    Mul(u32, u32),
    Div(u32, u32),
    Num(i64),
}

const MAX_MONKEY: usize = 4096;

pub fn main() -> Result<()> {
    let lines = read_lines("data/a21.txt")?;
    let mut dict = HashMap::<&str, usize>::with_capacity(lines.len());
    let mut next_dict_id = Cell::<usize>::default();

    let mut next_id = || {
        let tmp = next_dict_id.get();
        next_dict_id.set(tmp + 1);
        tmp
    };

    dict.insert("root", next_id());

    let mut monkeys = vec![None; MAX_MONKEY];

    for line in &lines {
        let mut split = line.splitn(2, ": ");
        let name = split.next().ok_or(Error::EmptyIterator)?;
        let id = *dict.entry(name).or_insert_with(next_id);
        let operation = split.next().ok_or(Error::EmptyIterator)?;

        let mut split = operation.splitn(3, ' ');
        let left = split.next().ok_or(Error::EmptyIterator)?;
        let monkey = match split.next() {
            None => Monkey::Num(left.parse()?),
            Some(operator) => {
                let right = split.next().ok_or(Error::EmptyIterator)?;
                let left_id = *dict.entry(left).or_insert_with(next_id);
                let right_id = *dict.entry(right).or_insert_with(next_id);

                match operator {
                    "+" => Monkey::Add(left_id as _, right_id as _),
                    "-" => Monkey::Sub(left_id as _, right_id as _),
                    "*" => Monkey::Mul(left_id as _, right_id as _),
                    "/" => Monkey::Div(left_id as _, right_id as _),
                    _ => return Err(Error::General(format!("Unsupported operator {operator}"))),
                }
            }
        };
        monkeys[id] = Some(monkey);
    }

    println!("part1: {}", solve_part1(&mut monkeys, 0));
    // dbg!(&monkeys[0..10]);

    Ok(())
}

fn solve_part1(monkeys: &mut [Option<Monkey>], i: usize) -> i64 {
    if let Some(monkey) = std::mem::take(&mut monkeys[i]) {
        let n = match monkey {
            Monkey::Num(n) => n,
            Monkey::Add(l, r) => solve_part1(monkeys, l as _) + solve_part1(monkeys, r as _),
            Monkey::Sub(l, r) => solve_part1(monkeys, l as _) - solve_part1(monkeys, r as _),
            Monkey::Mul(l, r) => solve_part1(monkeys, l as _) * solve_part1(monkeys, r as _),
            Monkey::Div(l, r) => solve_part1(monkeys, l as _) / solve_part1(monkeys, r as _),
        };

        monkeys[i] = Some(Monkey::Num(n));
        n
    } else {
        panic!("Not a monkey: {i}")
    }
}
