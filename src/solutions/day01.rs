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
        Ok(input.iter().map(|x| get_numbers(x)).sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        Ok(input.iter().map(|x| get_str_numbers(x)).sum())
    }
}

fn get_numbers(input: &str) -> usize {
    let first_digit = input.chars().find_map(|c| c.to_digit(10)).unwrap() as usize;
    let last_digit = input.chars().rev().find_map(|c| c.to_digit(10)).unwrap() as usize;
    return first_digit * 10 + last_digit;
}

static NUMBER_NAMES: &'static [&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

struct Trie {
    refs: Vec<String>,
    heads: Vec<(usize, usize)>,
}

impl Trie {
    fn new(refs: Vec<String>) -> Self {
        Self {
            refs,
            heads: Vec::new(),
        }
    }

    fn next(self: &mut Trie, c: char) -> Option<usize> {
        let mut new_heads = Vec::new();

        for (i, h) in self.heads.iter() {
            let str = &self.refs[*i];
            if str[*h..].starts_with(c) {
                if str.len() == h + 1 {
                    return Some(*i);
                }
                new_heads.push((*i, h + 1));
            }
        }

        for (i, h) in self.refs.iter().enumerate() {
            if h.starts_with(c) {
                new_heads.push((i, 1));
            }
        }

        self.heads = new_heads;

        None
    }
}

fn get_str_numbers(input: &str) -> usize {
    let mut forward_trie = Trie::new(NUMBER_NAMES.iter().map(|x| x.to_string()).collect());

    let mut backwards_trie = Trie::new(
        NUMBER_NAMES
            .iter()
            .map(|x| x.chars().rev().collect())
            .collect(),
    );
    let reversed_input: String = input.chars().rev().collect();

    let r = get_trie_digit(input, &mut forward_trie) * 10
        + get_trie_digit(&reversed_input, &mut backwards_trie);
    return r;
}

fn get_trie_digit(input: &str, trie: &mut Trie) -> usize {
    input
        .chars()
        .find_map(|c| {
            if c.is_digit(10) {
                c.to_digit(10).map(|v| v as usize)
            } else {
                trie.next(c).map(|v| v + 1)
            }
        })
        .unwrap()
}
