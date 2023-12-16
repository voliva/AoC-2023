use itertools::Itertools;

use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::str::FromStr;

pub struct Problem;

#[derive(Debug)]
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

fn print_posibility(p: &Vec<bool>) -> String {
    p.iter()
        .map(|v| match v {
            true => '#',
            false => '.',
        })
        .collect()
}

impl Report {
    fn to_string(self: &Self) -> String {
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

    fn count_posibilities(self: &Self) -> Vec<Vec<bool>> {
        // println!("{}", self.to_string());
        let r = self.count_posibilities_dbg();
        // println!("{} => {}", self.to_string(), r.len());
        return r;
    }
    fn count_posibilities_dbg(self: &Self) -> Vec<Vec<bool>> {
        let space_needed = self.get_total_contiguous() + self.contiguous.len() - 1;
        if self.operational.len() < space_needed {
            return Vec::new();
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
                .collect();
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
            return equivalent
                .count_posibilities()
                .iter()
                .map(|n| {
                    (0..first)
                        .map(|_| true)
                        .chain((0..1).map(|_| false))
                        .chain(n.iter().map(|v| *v))
                        .collect()
                })
                .collect();
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
            return equivalent
                .count_posibilities()
                .iter()
                .map(|n| {
                    (0..count)
                        .map(|_| false)
                        .chain(n.iter().map(|v| *v))
                        .collect()
                })
                .collect();
        }

        let remaining_space = self.operational.len()
            - (self.get_total_contiguous() - first + self.contiguous.len() - 1)
            - first
            + 1;
        if remaining_space == 0 {
            return Vec::new();
        }

        return (0..(remaining_space))
            .flat_map(|v| {
                if (0..v)
                    .any(|i| self.operational[i].is_some() && self.operational[i].unwrap() == true)
                {
                    return Vec::new();
                }
                if (v..(v + first))
                    .any(|i| self.operational[i].is_some() && self.operational[i].unwrap() == false)
                {
                    return Vec::new();
                }
                let space = v + first;
                if self.operational[space].is_some() && self.operational[space].unwrap() == true {
                    return Vec::new();
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
                let next_result = next.count_posibilities();

                return next_result
                    .iter()
                    .map(|n| {
                        (0..v)
                            .map(|_| false)
                            .chain((v..(v + first)).map(|_| true))
                            .chain((0..1).map(|_| false))
                            .chain(n.iter().map(|v| *v))
                            .collect()
                    })
                    .collect();
            })
            .collect();
    }

    fn get_total_contiguous(self: &Self) -> usize {
        self.contiguous
            .iter()
            .map(|v| *v)
            .reduce(|a, b| a + b)
            .unwrap()
    }

    fn verify(self: &Self, perm: &Vec<bool>) -> bool {
        if !zip(self.operational.iter(), perm).all(|(v, r)| match v {
            Some(op) => op == r,
            None => true,
        }) {
            return false;
        }

        let str = print_posibility(perm);
        let groups = str
            .split(".")
            .filter(|v| v.len() != 0)
            .map(|v| v.len())
            .collect_vec();

        return groups == self.contiguous;
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
        Ok(input
            .iter()
            .map(|v| {
                let r = v.count_posibilities();
                let strings = r.iter().map(|v| print_posibility(v)).collect_vec();
                let string_set: HashSet<String> = strings.iter().map(|v| v.to_owned()).collect();

                if strings.len() != string_set.len() {
                    println!("{}", v.to_string());
                    for p in &r {
                        println!("{}", print_posibility(p));
                    }
                    println!("");
                }
                for fail in r.iter().filter(|it| !v.verify(it)) {
                    println!("{}\n{}", v.to_string(), print_posibility(fail));
                }
                return r.len();
            })
            .reduce(|a, b| a + b)
            .unwrap())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        todo!()
    }
}
