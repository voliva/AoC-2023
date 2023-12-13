use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Vec<isize>>;
    type Output1 = isize;
    type Output2 = isize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.split(" ").map(|v| v.parse().unwrap()).collect())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input.iter().map(|v| find_next_value(v)).sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        Ok(input.iter().map(|v| find_prev_value(v)).sum())
    }
}

fn find_next_value(input: &Vec<isize>) -> isize {
    if input.len() == 0 {
        panic!("Empty array")
    }

    if input.iter().all(|v| *v == 0) {
        0
    } else {
        let mut diff = Vec::new();
        for i in 1..input.len() {
            diff.push(input[i] - input[i - 1]);
        }
        let value = find_next_value(&diff);
        input[input.len() - 1] + value
    }
}

fn find_prev_value(input: &Vec<isize>) -> isize {
    if input.len() == 0 {
        panic!("Empty array")
    }

    if input.iter().all(|v| *v == 0) {
        0
    } else {
        let mut diff = Vec::new();
        for i in 1..input.len() {
            diff.push(input[i] - input[i - 1]);
        }
        let value = find_prev_value(&diff);
        input[0] - value
    }
}
