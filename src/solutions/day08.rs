use itertools::Itertools;
use num::integer::lcm;
use num::Integer;

use super::Solver;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

enum Instruction {
    Left,
    Right,
}

pub struct Map {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, (String, String)>,
}

impl Solver for Problem {
    type Input = Map;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();

        let instructions = lines[0]
            .chars()
            .map(|c| {
                if c == 'R' {
                    Instruction::Right
                } else {
                    Instruction::Left
                }
            })
            .collect();
        let nodes = lines[2..].iter().fold(HashMap::default(), |mut acc, line| {
            let (source, routes) = line.split(" = ").collect_tuple().unwrap();
            let (left, right) = routes.split(", ").collect_tuple().unwrap();

            acc.insert(
                source.to_owned(),
                (
                    String::from(&left[1..]),
                    String::from(&right[..right.len() - 1]),
                ),
            );

            acc
        });

        Map {
            instructions,
            nodes,
        }
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut position = "AAA";
        let mut steps = 0;
        while position != "ZZZ" {
            let instruction = &input.instructions[steps % input.instructions.len()];
            let (left, right) = input.nodes.get(position).unwrap();
            position = match instruction {
                Instruction::Left => left,
                Instruction::Right => right,
            };
            steps += 1;
        }

        Ok(steps)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let positions = input
            .nodes
            .keys()
            .filter(|k| k.ends_with("A"))
            .collect_vec();

        Ok(positions
            .iter()
            .map(|p| find_loop(input, p))
            .reduce(|a, b| a.lcm(&b))
            .unwrap())
    }
}

fn find_loop(input: &Map, start: &str) -> usize {
    let mut visited: HashSet<(usize, &str)> = HashSet::new();
    let mut position = start;
    let mut exits = Vec::new();
    let mut steps = 0;

    while !visited.contains(&(steps % input.instructions.len(), position)) {
        visited.insert((steps % input.instructions.len(), position));

        let instruction = &input.instructions[steps % input.instructions.len()];
        let (left, right) = input.nodes.get(position).unwrap();
        position = match instruction {
            Instruction::Left => left,
            Instruction::Right => right,
        };
        steps += 1;
        if position.ends_with("Z") {
            exits.push(steps);
        }
    }

    exits.into_iter().reduce(|a, b| a.lcm(&b)).unwrap()
}
