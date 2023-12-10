use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::thread::current;

pub struct Problem;

#[derive(Debug)]
struct Mapping {
    source: usize,
    destination: usize,
    range: usize,
}

impl FromStr for Mapping {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (destination, source, range) = s
            .split(" ")
            .collect_tuple()
            .ok_or("Unkown mapping format")?;

        return Ok(Mapping {
            source: source.parse().unwrap(),
            destination: destination.parse().unwrap(),
            range: range.parse().unwrap(),
        });
    }
}

impl Mapping {
    fn map(self: &Self, v: usize) -> Option<usize> {
        if v >= self.source && v < self.source + self.range {
            Some(self.destination + v - self.source)
        } else {
            None
        }
    }
}

pub struct Almanac {
    seeds: Vec<usize>,
    mappings: Vec<Vec<Mapping>>,
}

impl Almanac {
    fn get_seed_location(self: &Self, seed: usize) -> usize {
        self.mappings.iter().fold(seed, |acc, mappings| {
            mappings
                .iter()
                .find_map(|mapping| mapping.map(acc))
                .unwrap_or(acc)
        })
    }
}

impl Solver for Problem {
    type Input = Almanac;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();

        let (_, seed_list) = lines[0].split(": ").collect_tuple().unwrap();
        let seeds = seed_list.split(" ").map(|x| x.parse().unwrap()).collect();

        let mut mappings = Vec::new();
        let mut current_mapping = Vec::new();
        for i in 3..lines.len() {
            if lines[i].contains("map") {
                mappings.push(current_mapping);
                current_mapping = Vec::new();
            } else if lines[i] != "" {
                current_mapping.push(lines[i].parse().unwrap());
            }
        }
        mappings.push(current_mapping);

        Almanac { seeds, mappings }
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input
            .seeds
            .iter()
            .map(|seed| input.get_seed_location(*seed))
            .min()
            .unwrap())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        todo!()
    }
}
