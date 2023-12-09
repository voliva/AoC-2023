use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<String>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader.lines().map(|x| x.unwrap()).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok((0..input.len())
            .flat_map(|line| get_line_nums(input, line))
            .sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        Ok((0..input.len())
            .map(|line: usize| find_gears(input, line))
            .sum())
    }
}

fn find_gears(grid: &Vec<String>, line: usize) -> usize {
    let mut result = 0;

    let mut idx = 0;
    while let Some(offset) = grid[line][idx..].find('*') {
        let gear_idx = idx + offset;
        let part_numbers = find_part_numbers(grid, line, gear_idx);
        if part_numbers.len() == 2 {
            // println!("{:?}", part_numbers);
            result = result + part_numbers.into_iter().reduce(|a, b| a * b).unwrap();
        }

        idx = gear_idx + 1;
    }

    result
}

fn find_part_numbers(grid: &Vec<String>, line: usize, col: usize) -> Vec<usize> {
    let mut result = Vec::new();
    let line = line as isize;
    let col = col as isize;

    let mut c = col - 1;
    // Top row
    while c <= col + 1 {
        if let Some((num, end)) = read_whole_number(grid, line - 1, c) {
            result.push(num);
            c = end as isize + 1;
        } else {
            c += 1;
        }
    }
    // Left
    if let Some((num, _)) = read_whole_number(grid, line, col - 1) {
        result.push(num);
    }

    // Right
    if let Some((num, _)) = read_whole_number(grid, line, col + 1) {
        result.push(num);
    }

    let mut c = col - 1;
    // Bottom row
    while c <= col + 1 {
        if let Some((num, end)) = read_whole_number(grid, line + 1, c) {
            result.push(num);
            c = end as isize + 1;
        } else {
            c += 1;
        }
    }

    return result;
}

// (num, end)
fn read_whole_number(grid: &Vec<String>, line: isize, col: isize) -> Option<(usize, usize)> {
    if line < 0 || (line as usize) > grid.len() || col < 0 || (col as usize) > grid[0].len() {
        return None;
    }
    let line = line as usize;
    let col = col as usize;

    // Find start
    let start = grid[line][..(col + 1)]
        .chars()
        .rev()
        .find_position(|c| !c.is_digit(10))
        .map(|(p, _)| col - p + 1)
        .unwrap_or(0);
    if start == col + 1 {
        return None;
    }

    let end = grid[line][start..]
        .chars()
        .find_position(|c| !c.is_digit(10))
        .map(|(p, _)| p)
        .unwrap_or(grid[line].len() - start);
    let num = grid[line][start..(start + end)].parse().unwrap();

    Some((num, start + end))
}

//////

fn get_line_nums(grid: &Vec<String>, line: usize) -> Vec<usize> {
    let mut result = Vec::new();

    let mut idx = 0;
    while let Some((num, sub_idx, len)) = find_num(&grid[line][idx..]) {
        let range = cap_col_range(
            grid,
            ((idx + sub_idx) as isize - 1)..(idx + sub_idx + len + 1) as isize,
        );
        let line = line as isize;
        if has_symbol(&grid[cap_line(grid, line - 1)][range.clone()])
            || has_symbol(&grid[cap_line(grid, line)][range.clone()])
            || has_symbol(&grid[cap_line(grid, line + 1)][range])
        {
            result.push(num);
        }

        idx += sub_idx + len;
    }

    result
}

// number index length
fn find_num(line: &str) -> Option<(usize, usize, usize)> {
    find_digit_idx(line)
        .and_then(|start| {
            find_nondigit_idx(&line[start..])
                .or(Some(line.len() - start))
                .map(|len| (start, len))
        })
        .map(|(start, len)| (line[start..start + len].parse().unwrap(), start, len))
}

fn find_digit_idx(line: &str) -> Option<usize> {
    line.char_indices()
        .find(|(_, c)| c.is_digit(10))
        .map(|(i, _)| i)
}

fn find_nondigit_idx(line: &str) -> Option<usize> {
    line.char_indices()
        .find(|(_, c)| !c.is_digit(10))
        .map(|(i, _)| i)
}

fn has_symbol(line: &str) -> bool {
    line.chars().any(|c| !c.is_digit(10) && c != '.')
}

fn cap_line(grid: &Vec<String>, line: isize) -> usize {
    (line.max(0) as usize).min(grid.len() - 1)
}
fn cap_col_range(grid: &Vec<String>, range: Range<isize>) -> Range<usize> {
    cap_col(grid, range.start)..cap_col(grid, range.end)
}
fn cap_col(grid: &Vec<String>, col: isize) -> usize {
    (col.max(0) as usize).min(grid[0].len() - 1)
}
