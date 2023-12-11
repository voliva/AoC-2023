use super::Solver;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::ops::Range;

pub struct Problem;

pub struct Race {
    time: usize,
    record: usize,
}

impl Race {
    pub fn get_winning_range(self: &Self) -> Range<usize> {
        let a = -1 as f64;
        let b = self.time as f64;
        let c = -(self.record as f64);

        let b2_4ac = b * b - 4. * a * c;
        let p_min = (-b + b2_4ac.sqrt()) / (2. * a);
        let p_max = (-b - b2_4ac.sqrt()) / (2. * a);

        let p_min = (p_min + 1.).floor() as usize;
        let p_max = (p_max - 1.).ceil() as usize;

        return p_min..(p_max + 1);
    }
}

impl Solver for Problem {
    type Input = (Vec<Race>, Race);
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines: Vec<String> = file_reader.lines().map(|x| x.unwrap()).collect_vec();
        let times = read_line(&lines[0]);
        let distances = read_line(&lines[1]);

        let races = zip(times, distances)
            .map(|(time, record)| Race { time, record })
            .collect();

        let race = Race {
            time: read_line_no_space(&lines[0]),
            record: read_line_no_space(&lines[1]),
        };

        (races, race)
    }

    fn solve_first(&self, (input, _): &Self::Input) -> Result<Self::Output1, String> {
        Ok(input.iter().map(|x| x.get_winning_range().len()).product())
    }

    fn solve_second(&self, (_, input): &Self::Input) -> Result<Self::Output2, String> {
        Ok(input.get_winning_range().len())
    }
}

fn read_line(line: &str) -> Vec<usize> {
    let (_, numbers) = line.split(":").collect_tuple().unwrap();

    numbers
        .split(" ")
        .filter(|v| !v.is_empty())
        .map(|v| v.parse().unwrap())
        .collect()
}

fn read_line_no_space(line: &str) -> usize {
    let (_, numbers) = line.split(":").collect_tuple().unwrap();

    numbers.replace(" ", "").parse().unwrap()
}
