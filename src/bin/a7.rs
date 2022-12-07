use adventofcode2022::Result;
use adventofcode2022::{read_lines, Error};
use std::cell::Cell;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Line {
    Cd(String),
    Ls,
    Dir(String),
    File(String, usize),
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(line: &str) -> std::prelude::rust_2015::Result<Self, Self::Err> {
        if let Some(rest) = line.strip_prefix("$ cd ") {
            return Ok(Line::Cd(rest.to_string()));
        }
        if let Some(_) = line.strip_prefix("$ ls") {
            return Ok(Line::Ls);
        }
        if let Some(dir) = line.strip_prefix("dir ") {
            return Ok(Line::Dir(dir.to_string()));
        }
        let mut split = line.splitn(2, ' ');
        let size = split.next().ok_or(Error::PatternMatch)?.parse()?;
        let name = split.next().ok_or(Error::PatternMatch)?.to_string();
        Ok(Line::File(name, size))
    }
}

#[derive(Debug)]
enum Node {
    File(String, usize),
    Dir(String, Vec<Node>, Cell<Option<usize>>),
}

impl Node {
    fn name(&self) -> &str {
        match self {
            Node::File(name, _) => name,
            Node::Dir(name, _, _) => name,
        }
    }
    fn total_size(&self) -> usize {
        match self {
            Node::File(_, size) => *size,
            Node::Dir(_name, children, cached_size) => {
                if let Some(size) = cached_size.get() {
                    size
                } else {
                    let size = children.iter().map(Node::total_size).sum();
                    cached_size.set(Some(size));
                    size
                }
            }
        }
    }
}

impl FromIterator<Line> for Node {
    fn from_iter<T: IntoIterator<Item = Line>>(iter: T) -> Self {
        fn collect<I: Iterator<Item = Line>>(iter: &mut I, nodes: &mut Vec<Node>) {
            loop {
                match iter.next() {
                    None => break,
                    Some(Line::Ls) => {
                        // assume this only occurs once at the beginning of a dir and ignore
                    }
                    Some(Line::Cd(dir)) => {
                        if dir == ".." {
                            break;
                        } else {
                            let mut children = vec![];
                            collect(iter, &mut children);
                            nodes.push(Node::Dir(dir, children, Cell::new(None)))
                        }
                    }
                    Some(Line::File(name, size)) => nodes.push(Node::File(name, size)),
                    Some(Line::Dir(_name)) => {
                        // nodes.push(Node::Dir(name, size))
                    }
                }
            }
        }

        let mut nodes = vec![];
        let mut iter1 = iter.into_iter();
        collect(&mut iter1, &mut nodes);

        Node::Dir("".into(), nodes, Cell::new(None))
    }
}

fn solve_part1(root: &Node) -> usize {
    fn solve_part1_helper(root: &Node, sum: &mut usize) {
        match root {
            dir @ Node::Dir(_, children, _) => {
                let size = dir.total_size();
                if size <= 100_000 {
                    *sum += size;
                }
                for child in children {
                    solve_part1_helper(child, sum);
                }
            }
            _ => {}
        }
    }
    let mut sum = 0;
    solve_part1_helper(root, &mut sum);
    sum
}

fn solve_part2(root: &Node) -> Option<(String, usize)> {
    fn collect_dirs(node: &Node, dirs: &mut Vec<(String, usize)>) {
        match node {
            Node::Dir(_, children, _) => {
                dirs.push((node.name().to_string(), node.total_size()));
                for child in children {
                    collect_dirs(child, dirs);
                }
            }
            Node::File(_, _) => {}
        }
    }

    let root_size = root.total_size();

    let mut dirs = vec![];
    collect_dirs(root, &mut dirs);

    let dirs = dirs
        .into_iter()
        .filter(|(_, size)| 70_000_000 - root_size + *size >= 30_000_000)
        .collect::<Vec<_>>();
    // dbg!(&dirs);
    // dirs.sort_by(|(_, size)| *size);
    dirs.into_iter().min_by_key(|(_, size)| *size)
}

fn main() -> Result<()> {
    let mut lines = read_lines("data/a7.txt")?
        .into_iter()
        .map(|line| Line::from_str(&line))
        .collect::<Result<Vec<Line>>>()?;

    // assume `cd /` only occurs once at the beginning
    assert_eq!(lines.remove(0), Line::Cd("/".into()));

    let root = Node::from_iter(lines.into_iter());

    println!("part1: {}", solve_part1(&root));
    println!("part2: {}", solve_part2(&root).unwrap().1);

    Ok(())
}
