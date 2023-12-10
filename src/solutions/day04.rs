use itertools::Itertools;

use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Problem;

pub struct Card {
    winning: HashSet<u8>,
    owning: HashSet<u8>,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let formatted = s.replace("  ", " ");
        let (_, lists) = formatted
            .split(": ")
            .collect_tuple()
            .ok_or("Unknown card fromat")?;
        let (winning_list, owning_list) = lists
            .split(" | ")
            .collect_tuple()
            .ok_or("Unknown card fromat")?;

        let winning = winning_list
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();
        let owning = owning_list.split(" ").map(|x| x.parse().unwrap()).collect();

        Ok(Card { winning, owning })
    }
}

impl Card {
    fn get_matches(self: &Self) -> usize {
        self.owning
            .iter()
            .filter(|x| self.winning.contains(x))
            .count()
    }

    fn get_points(self: &Self) -> usize {
        let matches = self.get_matches();

        if matches == 0 {
            0
        } else {
            (2_usize).pow(matches as u32 - 1)
        }
    }
}

impl Solver for Problem {
    type Input = Vec<Card>;
    type Output1 = usize;
    type Output2 = u64;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.parse())
            .map(|x| x.unwrap())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input.iter().map(|card| card.get_points()).sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut quantities: Vec<u64> = input.iter().map(|_| 1).collect();

        for i in 0..input.len() {
            let matches = input[i].get_matches();
            for m in 0..matches {
                quantities[i + m + 1] += quantities[i];
            }
        }

        Ok(quantities.iter().sum())
    }
}
