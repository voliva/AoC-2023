use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Problem;

#[derive(Debug, Default)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for Draw {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Draw::default();
        for color_quantity in s.split(", ") {
            let (quantity, color) = color_quantity
                .split(" ")
                .collect_tuple()
                .ok_or("Unknown color quantity format")?;
            let q: usize = quantity.parse().map_err(|_| "Quantity not parsable")?;
            match color {
                "red" => result.red = q,
                "green" => result.green = q,
                "blue" => result.blue = q,
                _ => return Err(format!("Unknown color {color}")),
            }
        }

        Ok(result)
    }
}

impl Draw {
    fn fits_in(self: &Self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn union(self: &Self, other: &Self) -> Self {
        Draw {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn get_power(self: &Self) -> usize {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
pub struct Game {
    sets: Vec<Draw>,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, draws) = s.split(": ").collect_tuple().unwrap();

        Ok(Game {
            sets: draws
                .split("; ")
                .map(|str| Draw::from_str(str).unwrap())
                .collect(),
        })
    }
}

impl Solver for Problem {
    type Input = Vec<Game>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.parse())
            .map(|x| x.unwrap())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let max = Draw {
            red: 12,
            green: 13,
            blue: 14,
        };

        Ok(input
            .iter()
            .enumerate()
            .filter(|(_, game)| game.sets.iter().all(|set| set.fits_in(&max)))
            .map(|(i, _)| i + 1)
            .sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        Ok(input
            .iter()
            .map(|game| {
                game.sets
                    .iter()
                    .fold(Draw::default(), |a, b| a.union(b))
                    .get_power()
            })
            .sum())
    }
}
