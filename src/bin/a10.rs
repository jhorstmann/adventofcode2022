use adventofcode2022::Result;
use adventofcode2022::{read_lines, Error};
use std::str::from_utf8;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    AddX(i64),
    Noop,
}

pub fn main() -> Result<()> {
    let lines = read_lines("data/a10.txt")?
        .iter()
        .map(|line| {
            if line == "noop" {
                return Ok(Instruction::Noop);
            }
            if let Some(y) = line.strip_prefix("addx ") {
                return Ok(Instruction::AddX(y.parse()?));
            }
            return Err(Error::PatternMatch);
        })
        .collect::<Result<Vec<Instruction>>>()?;

    let mut values = vec![0_i64; 255];
    values[0] = 1;

    let mut cycle = 0;
    let mut last_value = 1;
    for instr in lines {
        match instr {
            Instruction::Noop => {
                values[cycle] = last_value;
                cycle += 1;
            }
            Instruction::AddX(x) => {
                values[cycle] = last_value;
                cycle += 1;
                values[cycle] = last_value;
                cycle += 1;
                last_value += x;
            }
        }
        // println!("{cycle:3}: {instr:?} => {last_value}");
        // if cycle > 221 {
        //     break;
        // }
    }

    values.truncate(cycle);

    let part1 = (20..221).step_by(40).fold(0_i64, |mut acc, i| {
        let v = values[i - 1];
        let tmp = (i as i64) * v;
        acc += tmp;
        acc
    });

    println!("part1: {part1}");

    let display = values
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let x = *x as i64;
            let i = (i as i64) % 40;
            if x >= (i - 1) && x <= (i + 1) {
                b'#'
            } else {
                b'.'
            }
        })
        .take(40 * 6)
        .collect::<Vec<u8>>();

    display.chunks(40).for_each(|line| {
        println!("{}", from_utf8(line).unwrap());
    });

    Ok(())
}
