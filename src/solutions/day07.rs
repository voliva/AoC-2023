use itertools::Itertools;

use super::Solver;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::str::FromStr;

pub struct Problem;

#[derive(Clone, Debug)]
pub struct Hand {
    cards: Vec<u32>,
    bid: usize,
    strength: usize,
}

impl Hand {
    fn new(cards: Vec<u32>, bid: usize) -> Self {
        let card_counts: HashMap<u32, usize> =
            cards
                .iter()
                .filter(|v| **v != 1)
                .fold(HashMap::default(), |mut acc, c| {
                    if let Some(v) = acc.get(c) {
                        acc.insert(*c, v + 1);
                    } else {
                        acc.insert(*c, 1);
                    }
                    return acc;
                });

        // exclude jokers from max_count, as they are added to the maximum
        let max_count = card_counts.values().max().unwrap_or(&0);
        let jokers = cards.iter().filter(|v| **v == 1).count();

        let strength = match max_count + jokers {
            1 => 1,
            2 => {
                let twos = card_counts.values().filter(|v| **v == 2).count();
                if twos == 1 || twos == 0 {
                    2
                } else {
                    3
                }
            }
            3 => {
                let twos = card_counts.values().filter(|v| **v == 2).count();

                match (jokers, twos) {
                    (0, 0) => 4,
                    (0, 1) => 5,
                    (1, 1) => 4,
                    (1, 2) => 5,
                    (2, _) => 4,
                    _ => unreachable!(),
                }
            }
            4 => 6,
            5 => 7,
            _ => unreachable!(),
        };

        Hand {
            cards,
            bid,
            strength,
        }
    }

    fn _get_hand(self: &Self) -> String {
        let result: String = self
            .cards
            .iter()
            .map(|v| char::from_digit(*v, 16).unwrap().to_ascii_uppercase())
            .collect();
        result
            .replace("A", "T")
            .replace("1", "J")
            .replace("B", "J")
            .replace("C", "Q")
            .replace("D", "K")
            .replace("E", "A")
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card_list, bid) = s.split(" ").collect_tuple().ok_or("Unknown hand format")?;

        Ok(Hand::new(
            card_list
                .replace("A", "E")
                .replace("T", "A")
                .replace("J", "1") // TODO "B" instead of "1" for part 1
                .replace("Q", "C")
                .replace("K", "D")
                .chars()
                .map(|c| c.to_digit(16).unwrap())
                .collect(),
            bid.parse().map_err(|_| "Can't parse bid")?,
        ))
    }
}

impl Solver for Problem {
    type Input = Vec<Hand>;
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
        let mut input = input.clone();
        input.sort_by(|a, b| {
            let strength_result = a.strength.cmp(&b.strength);
            if strength_result != Ordering::Equal {
                return strength_result;
            }

            zip(&a.cards, &b.cards)
                .find_map(|(a, b)| {
                    let card_result = a.cmp(&b);
                    if card_result == Ordering::Equal {
                        None
                    } else {
                        Some(card_result)
                    }
                })
                .unwrap()
        });

        Ok(input
            .iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i + 1))
            .sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        self.solve_first(input)
    }
}
