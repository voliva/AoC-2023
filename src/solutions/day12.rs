use itertools::Itertools;

use super::Solver;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::str::FromStr;

pub struct Problem;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Report {
    operational: Vec<Option<bool>>,
    contiguous: Vec<usize>,
}

impl FromStr for Report {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (operational, contiguous) = s.split(" ").collect_tuple().ok_or("Unkown format")?;

        Ok(Report {
            operational: operational
                .chars()
                .map(|v| match v {
                    '#' => Some(true),
                    '.' => Some(false),
                    '?' => None,
                    _ => unreachable!(),
                })
                .collect(),
            contiguous: contiguous.split(",").map(|v| v.parse().unwrap()).collect(),
        })
    }
}

fn _print_posibility(p: &Vec<bool>) -> String {
    p.iter()
        .map(|v| match v {
            true => '#',
            false => '.',
        })
        .collect()
}

impl Report {
    fn _to_string(self: &Self) -> String {
        let operational: String = self
            .operational
            .iter()
            .map(|v| match v {
                Some(true) => '#',
                Some(false) => '.',
                None => '?',
            })
            .collect();
        let contiguous = self.contiguous.iter().join(",");
        format!("{operational} {contiguous}")
    }

    fn count_posibilities(self: &Self, memo: &mut HashMap<Report, usize>) -> usize {
        if let Some(r) = memo.get(self) {
            return *r;
        }
        let r = self.count_posibilities_memo(memo);
        memo.insert(self.to_owned(), r);
        return r;
    }
    fn count_posibilities_memo(self: &Self, memo: &mut HashMap<Report, usize>) -> usize {
        let space_needed = self.get_total_contiguous() + self.contiguous.len() - 1;
        if self.operational.len() < space_needed {
            return 0;
        }

        let first = self.contiguous[0];
        if self.contiguous.len() == 1 {
            return get_permutations(self.operational.len(), first)
                .into_iter()
                .filter(|perm| {
                    zip(perm, &self.operational).all(|(p, o)| match o {
                        Some(v) => *p == *v,
                        None => true,
                    })
                })
                .count();
        }

        let first_fits = (0..first + 1).all(|i| match self.operational[i] {
            Some(true) => i < first,
            Some(false) => i == first,
            None => i > 0,
        });
        if first_fits {
            let equivalent = Report {
                operational: self
                    .operational
                    .iter()
                    .skip(first + 1)
                    .map(|v| v.to_owned())
                    .collect(),
                contiguous: self
                    .contiguous
                    .iter()
                    .skip(1)
                    .map(|v| v.to_owned())
                    .collect(),
            };
            return equivalent.count_posibilities(memo);
        }
        if let Some(false) = self.operational[0] {
            let count = self
                .operational
                .iter()
                .take_while(|v| v.is_some() && v.unwrap() == false)
                .count();

            let equivalent = Report {
                operational: self
                    .operational
                    .iter()
                    .skip(count)
                    .map(|v| v.to_owned())
                    .collect(),
                contiguous: self.contiguous.clone(),
            };
            return equivalent.count_posibilities(memo);
        }

        let remaining_space = self.operational.len()
            - (self.get_total_contiguous() - first + self.contiguous.len() - 1)
            - first
            + 1;
        if remaining_space == 0 {
            return 0;
        }

        return (0..(remaining_space))
            .map(|v| {
                if (0..v)
                    .any(|i| self.operational[i].is_some() && self.operational[i].unwrap() == true)
                {
                    return 0;
                }
                if (v..(v + first))
                    .any(|i| self.operational[i].is_some() && self.operational[i].unwrap() == false)
                {
                    return 0;
                }
                let space = v + first;
                if self.operational[space].is_some() && self.operational[space].unwrap() == true {
                    return 0;
                }

                let next = Report {
                    operational: self
                        .operational
                        .iter()
                        .skip(first + v + 1)
                        .map(|v| v.to_owned())
                        .collect(),
                    contiguous: self
                        .contiguous
                        .iter()
                        .skip(1)
                        .map(|v| v.to_owned())
                        .collect(),
                };
                return next.count_posibilities(memo);
            })
            .reduce(|a, b| a + b)
            .unwrap();
    }

    fn get_total_contiguous(self: &Self) -> usize {
        self.contiguous
            .iter()
            .map(|v| *v)
            .reduce(|a, b| a + b)
            .unwrap()
    }

    fn unfold(self: &Self) -> Self {
        Report {
            contiguous: (0..5)
                .flat_map(|_| self.contiguous.iter().map(|v| *v).collect_vec())
                .collect(),
            operational: (0..5)
                .flat_map(|i| {
                    if i == 4 {
                        self.operational.iter().map(|v| v.to_owned()).collect_vec()
                    } else {
                        self.operational
                            .iter()
                            .chain(vec![None].iter())
                            .map(|v| v.to_owned())
                            .collect_vec()
                    }
                })
                .collect(),
        }
    }
}

fn get_permutations(len: usize, n: usize) -> Vec<Vec<bool>> {
    (0..(len - n + 1))
        .map(|s| (0..len).map(|v| v >= s && v < s + n).collect())
        .collect()
}

impl Solver for Problem {
    type Input = Vec<Report>;
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
        let mut memo = HashMap::new();
        Ok(input
            .iter()
            .map(|v| v.count_posibilities(&mut memo))
            .reduce(|a, b| a + b)
            .unwrap())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut memo = HashMap::new();
        Ok(input
            .iter()
            .map(|v| v.unfold())
            .map(|v| v.count_posibilities(&mut memo))
            .reduce(|a, b| a + b)
            .unwrap())
    }
}
