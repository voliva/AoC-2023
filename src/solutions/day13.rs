use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

pub struct Problem;

#[derive(Default)]
pub struct Pattern {
    lines: Vec<String>,
}

enum Symmetry {
    Vertical(usize),
    Horizontal(usize),
}

impl Pattern {
    fn get_symmetry(self: &Self) -> Symmetry {
        self.get_horizontal_symmetry()
            .or_else(|| self.get_vertical_symmetry())
            .unwrap()
    }

    fn get_horizontal_symmetry(self: &Self) -> Option<Symmetry> {
        get_symmetry(&self.lines).map(|v| Symmetry::Horizontal(v))
    }

    fn get_vertical_symmetry(self: &Self) -> Option<Symmetry> {
        let transposed: Vec<String> = (0..self.lines[0].len())
            .map(|i| {
                self.lines
                    .iter()
                    .map(|s| s[i..(i + 1)].to_owned())
                    .collect()
            })
            .collect_vec();

        get_symmetry(&transposed).map(|v| Symmetry::Vertical(v))
    }
}

fn get_symmetry(lines: &Vec<String>) -> Option<usize> {
    let candidates = lines
        .iter()
        .tuple_windows()
        .enumerate()
        .filter_map(|(i, (a, b))| if a == b { Some(i) } else { None })
        .collect_vec();

    for c in candidates {
        let mut missmatch = false;
        for i in 0..(lines.len() - c - 1) {
            if i > c {
                return Some(c);
            }
            if lines[c - i] != lines[c + i + 1] {
                missmatch = true;
                break;
            }
        }
        if !missmatch {
            return Some(c);
        }
    }

    None
}

impl Solver for Problem {
    type Input = Vec<Pattern>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();

        let mut result = Vec::new();
        let mut current = Pattern::default();

        for line in lines {
            if line == "" {
                result.push(current);
                current = Pattern::default();
            } else {
                current.lines.push(line.to_owned());
            }
        }
        if current.lines.len() > 0 {
            result.push(current);
        }

        result
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input
            .iter()
            .map(|v| v.get_symmetry())
            .map(|v| match v {
                Symmetry::Vertical(v) => v + 1,
                Symmetry::Horizontal(v) => 100 * (v + 1),
            })
            .reduce(|a, b| a + b)
            .unwrap())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        todo!()
    }
}
