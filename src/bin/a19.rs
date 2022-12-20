use std::str::FromStr;
use adventofcode2022::{Error, read_lines, regex, Result};

#[derive(Debug, Clone)]
struct Blueprint([[usize; 3]; 4]);

#[repr(usize)]
enum Material {
    Ore = 0,
    Cly = 1,
    Obs = 2,
    Geo = 3,
}

impl FromStr for Blueprint {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let pattern = regex!("^Blueprint [0-9]+: Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.$");
        let captures = pattern.captures(s).ok_or(Error::PatternMatch)?;

        let ore_robot_cost = [captures[1].parse()?, 0, 0];
        let cly_robot_cost = [captures[2].parse()?, 0, 0];
        let obs_robot_cost = [captures[3].parse()?, captures[4].parse()?, 0];
        let geo_robot_cost = [captures[5].parse()?, 0, captures[6].parse()?];

        Ok(Blueprint([ore_robot_cost, cly_robot_cost, obs_robot_cost, geo_robot_cost]))
    }
}

pub fn main() -> Result<()> {
    let blueprints = read_lines("data/a19_example.txt")?.iter().map(|line| Blueprint::from_str(line)).collect::<Result<Vec<_>>>()?;

    dbg!(blueprints.iter().flat_map(|bp| bp.0.iter().flatten()).max());
    // dbg!(&blueprints);

    // Blueprint 1:
    //   Each ore robot costs 4 ore.
    //   Each clay robot costs 2 ore.
    //   Each obsidian robot costs 3 ore and 14 clay.
    //   Each geode robot costs 2 ore and 7 obsidian.

    let mut state = vec![[[[0_u64; 20]; 20]; 20]; 24];
    state[2][0][7][1] = 1;
    // ...
    state[2][0][7][24] = 24;


    state[2+7*3][7*14][0][2] = 1;
    // ...
    state[2+7*3][7*14][0][24] = 24;

    state[2+7*3+7*14*2][0][0][3] = 1;
    // ...

    state[2+7*3+7*14*2][0][0][3] = 1;


    state[2*3][7*14][0][0][1] = 1;



    Ok(())
}