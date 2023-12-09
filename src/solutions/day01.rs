use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<String>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader.lines().map(|x| x.unwrap()).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        input
            .iter()
            .map(|x| get_numbers(x))
            .reduce(|acc, x| acc + x)
            .ok_or("Empty input".to_string())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        // > 53914
        input
            .iter()
            .map(|x| get_str_numbers(x))
            .reduce(|acc, x| {
                // println!("{acc} + {x}");
                return acc + x;
            })
            .ok_or("Empty input".to_string())
    }
}

fn get_numbers(input: &str) -> usize {
    let only_digits: String = input.chars().filter(|c| c.is_digit(10)).collect();
    match only_digits.len() {
        0 => 0,
        l => {
            only_digits[..1].parse::<usize>().unwrap() * 10
                + only_digits[l - 1..].parse::<usize>().unwrap()
        }
    }
}

static NUMBER_NAMES: &'static [&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_str_numbers(input: &str) -> usize {
    let r = get_first_digit(input) * 10 + get_last_digit(input);
    return r;
}

fn get_first_digit(input: &str) -> usize {
    let mut possibilities = NUMBER_NAMES.iter().map(|_| 0).collect_vec();

    for c in input.chars() {
        let c_str = c.to_string();
        if c.is_digit(10) {
            return c_str.parse().unwrap();
        }

        for (i, p) in possibilities.clone().iter().enumerate() {
            if NUMBER_NAMES[i][*p..*p + 1] == c_str {
                if *p == NUMBER_NAMES[i].len() - 1 {
                    return i + 1;
                } else {
                    possibilities[i] = *p + 1;
                }
            } else {
                possibilities[i] = if NUMBER_NAMES[i][..1] == c_str { 1 } else { 0 };
            }
        }
    }

    panic!("Unreachable gfs {input}")
}

fn get_last_digit(input: &str) -> usize {
    let mut possibilities = NUMBER_NAMES.iter().map(|_| 0).collect_vec();

    for c in input.chars().rev() {
        let c_str = c.to_string();
        if c.is_digit(10) {
            return c_str.parse().unwrap();
        }

        for (i, p) in possibilities.clone().iter().enumerate() {
            let p_inv = NUMBER_NAMES[i].len() - p - 1;
            if NUMBER_NAMES[i][p_inv..p_inv + 1] == c_str {
                if *p == NUMBER_NAMES[i].len() - 1 {
                    return i + 1;
                } else {
                    possibilities[i] = *p + 1;
                }
            } else {
                let l = NUMBER_NAMES[i].len();
                possibilities[i] = if NUMBER_NAMES[i][l - 1..] == c_str {
                    1
                } else {
                    0
                };
            }
        }
    }

    panic!("Unreachable {input}")
}
