use super::Solver;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

pub struct Space {
    galaxies: Vec<(usize, usize)>,
    range: (usize, usize),
}

impl Space {
    fn expand(self: &Self, age: usize) -> Space {
        let (range_x, range_y) = &self.range;
        let mut empty_cols: HashSet<usize> = (0..*range_x).collect();
        let mut empty_rows: HashSet<usize> = (0..*range_y).collect();

        for (x, y) in &self.galaxies {
            empty_cols.remove(x);
            empty_rows.remove(y);
        }

        let galaxies = self
            .galaxies
            .iter()
            .map(|v| v.clone())
            .map(|(x, y)| {
                let x_expansion = empty_cols.iter().filter(|c| **c < x).count();
                let y_expansion = empty_rows.iter().filter(|r| **r < y).count();
                (x + x_expansion * age, y + y_expansion * age)
            })
            .collect();

        Space {
            galaxies,
            range: (
                *range_x + empty_cols.len() * age,
                *range_y + empty_rows.len() * age,
            ),
        }
    }

    fn get_distances_sum(self: &Self) -> usize {
        self.galaxies
            .iter()
            .combinations(2)
            .map(|v| v[0].0.abs_diff(v[1].0) + v[0].1.abs_diff(v[1].1))
            .sum()
    }
}

impl Solver for Problem {
    type Input = Space;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();

        let galaxies = lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, v)| *v == '#')
                    .map(|(x, _)| (x, y))
                    .collect_vec()
            })
            .collect();

        Space {
            galaxies,
            range: (lines[0].len(), lines.len()),
        }
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let expanded = input.expand(1);

        Ok(expanded.get_distances_sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let expanded = input.expand(1_000_000 - 1);

        Ok(expanded.get_distances_sum())
    }
}
