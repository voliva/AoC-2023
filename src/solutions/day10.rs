use itertools::Itertools;

use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

#[derive(PartialEq, Clone)]
pub enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    G,
    S,
}

impl TryFrom<char> for Pipe {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Pipe::NS),
            '-' => Ok(Pipe::EW),
            'L' => Ok(Pipe::NE),
            'J' => Ok(Pipe::NW),
            '7' => Ok(Pipe::SW),
            'F' => Ok(Pipe::SE),
            '.' => Ok(Pipe::G),
            'S' => Ok(Pipe::S),
            _ => Err("Unknown pipe".to_owned()),
        }
    }
}

impl Solver for Problem {
    type Input = Vec<Vec<Pipe>>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.chars().map(|c| c.try_into().unwrap()).collect())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let loop_pipes = get_loop(input);

        Ok(loop_pipes.len() / 2)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let loop_pipes = get_loop(input);
        let mut enclosed = get_initial_enclosed(input, &loop_pipes);
        let mut open: HashSet<(usize, usize)> = HashSet::new();

        for (y, line) in input.iter().enumerate() {
            for (x, pipe) in line.iter().enumerate() {
                let coord = (x, y);
                if *pipe != Pipe::G || enclosed.contains(&coord) || open.contains(&coord) {
                    continue;
                }

                if is_edge(input, coord) {
                    open.insert(coord);
                    continue;
                }

                let mut current_chain: HashSet<(usize, usize)> = HashSet::new();
                current_chain.insert(coord);
                let mut heads = Vec::new();
                heads.push(coord);

                while let Some(coord) = heads.pop() {
                    if is_edge(input, coord) || open.contains(&coord) {
                        for c in &current_chain {
                            open.insert(*c);
                        }
                        current_chain.clear();
                        break;
                    }
                    if enclosed.contains(&coord) {
                        for c in &current_chain {
                            enclosed.insert(*c);
                        }
                        current_chain.clear();
                        break;
                    }

                    for adj in get_adjacent_ground(input, coord) {
                        if current_chain.contains(&adj) {
                            continue;
                        }
                        heads.push(adj);
                        current_chain.insert(adj);
                    }
                }
            }
        }

        let grounds = enclosed
            .iter()
            .filter(|(x, y)| *y < input.len() && *x < input[*y].len())
            .filter(|(x, y)| input[*y][*x] == Pipe::G)
            .collect_vec();

        // < 580, != 63, != 93
        Ok(grounds.len())
    }
}

fn get_adjacent(
    pipes: &Vec<Vec<Pipe>>,
    (x, y): (usize, usize),
) -> Option<((usize, usize), (usize, usize))> {
    match pipes[y][x] {
        Pipe::NS => Some(((x + 0, y - 1), (x + 0, y + 1))),
        Pipe::EW => Some(((x + 1, y + 0), (x - 1, y + 0))),
        Pipe::NE => Some(((x + 0, y - 1), (x + 1, y + 0))),
        Pipe::NW => Some(((x + 0, y - 1), (x - 1, y + 0))),
        Pipe::SW => Some(((x + 0, y + 1), (x - 1, y + 0))),
        Pipe::SE => Some(((x + 0, y + 1), (x + 1, y + 0))),
        Pipe::S => {
            let mut connecting = Vec::new();
            if y > 0 {
                match pipes[y - 1][x] {
                    Pipe::NS | Pipe::SW | Pipe::SE => connecting.push((x, y - 1)),
                    _ => {}
                }
            }
            if y < pipes.len() - 1 {
                match pipes[y + 1][x] {
                    Pipe::NS | Pipe::NW | Pipe::NE => connecting.push((x, y + 1)),
                    _ => {}
                }
            }
            if x > 0 {
                match pipes[y][x - 1] {
                    Pipe::EW | Pipe::NE | Pipe::SE => connecting.push((x - 1, y)),
                    _ => {}
                }
            }
            if x < pipes[0].len() - 1 {
                match pipes[y][x + 1] {
                    Pipe::EW | Pipe::NW | Pipe::SW => connecting.push((x + 1, y)),
                    _ => {}
                }
            }

            return Some((connecting[0], connecting[1]));
        }
        Pipe::G => None,
    }
}

fn get_loop(pipes: &Vec<Vec<Pipe>>) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut position = pipes
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find(|(_, p)| **p == Pipe::S)
                .map(|(x, _)| (x, y))
        })
        .unwrap();
    while !visited.contains(&position) {
        visited.insert(position);
        let (a, b) = get_adjacent(pipes, position).unwrap();
        position = if visited.contains(&a) { b } else { a }
    }

    visited
}

fn get_initial_enclosed(
    pipes: &Vec<Vec<Pipe>>,
    loop_pipes: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();
    let mut visited = HashSet::new();
    let (mut coord, mut natural) = get_initial_pipe(pipes, loop_pipes);

    while !visited.contains(&coord) {
        visited.insert(coord);
        let (x, y) = coord;
        let pipe = pipes[y][x].clone();

        if natural {
            match pipe {
                Pipe::NS => {
                    result.insert((x + 1, y));
                }
                Pipe::EW => {
                    result.insert((x, y + 1));
                }
                Pipe::NE => {
                    result.insert((x + 1, y - 1));
                }
                Pipe::NW => {
                    result.insert((x + 1, y));
                    result.insert((x + 1, y + 1));
                    result.insert((x, y + 1));
                }
                Pipe::SW => {
                    result.insert((x + 1, y));
                    if y > 0 {
                        result.insert((x + 1, y - 1));
                        result.insert((x, y - 1));
                    }
                }
                Pipe::SE => {
                    result.insert((x + 1, y + 1));
                }
                _ => {}
            }
        } else {
            match pipe {
                Pipe::NS => {
                    if x > 0 {
                        result.insert((x - 1, y));
                    }
                }
                Pipe::EW => {
                    if y > 0 {
                        result.insert((x, y - 1));
                    }
                }
                Pipe::NE => {
                    if x > 0 {
                        result.insert((x - 1, y));
                        result.insert((x - 1, y + 1));
                    }
                    result.insert((x, y + 1));
                }
                Pipe::NW => {
                    if x > 0 {
                        result.insert((x - 1, y - 1));
                    }
                }
                Pipe::SW => {
                    if x > 0 {
                        result.insert((x - 1, y + 1));
                    }
                }
                Pipe::SE => {
                    if x > 0 {
                        result.insert((x - 1, y));

                        if y > 0 {
                            result.insert((x - 1, y - 1));
                        }
                    }
                    if y > 0 {
                        result.insert((x, y - 1));
                    }
                }
                _ => {}
            }
        }

        let (a, b) = get_adjacent(pipes, coord).unwrap();
        coord = if visited.contains(&a) { b } else { a };

        let (x, y) = coord;
        let next_pipe = pipes[y][x].clone();
        natural = match (pipe, next_pipe) {
            (Pipe::EW, Pipe::NE | Pipe::SE) => !natural,
            (Pipe::NE, Pipe::EW | Pipe::NW | Pipe::SW) => !natural,
            (Pipe::NW, Pipe::NE) => !natural,
            (Pipe::SW, Pipe::EW | Pipe::SE) => !natural,
            (Pipe::SE, Pipe::SW) => !natural,
            _ => natural,
        };
    }

    result
}

/*
natural
    NS right
    EW bottom
    NE inside
    NW outside
    SW outside
    SE inside,
 */
fn get_initial_pipe(
    pipes: &Vec<Vec<Pipe>>,
    loop_pipes: &HashSet<(usize, usize)>,
) -> ((usize, usize), bool) {
    for y in 0..pipes.len() {
        for x in 0..pipes[y].len() {
            if !loop_pipes.contains(&(x, y)) {
                continue;
            }
            match pipes[y][x] {
                // Pipe::NS => return ((x, y), true),
                // Pipe::EW => return ((x, y), true),
                // Pipe::NE => return ((x, y), true),
                // Pipe::NW => return ((x, y), true),
                // Pipe::SW => return ((x, y), false),
                Pipe::SE => return ((x, y), true),
                _ => {}
            }
        }
    }
    unreachable!()
}

fn get_adjacent_ground(pipes: &Vec<Vec<Pipe>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    if y > 0 && pipes[y - 1][x] == Pipe::G {
        result.push((x, y - 1));
    }
    if y < pipes.len() - 1 && pipes[y + 1][x] == Pipe::G {
        result.push((x, y + 1));
    }
    if x > 0 && pipes[y][x - 1] == Pipe::G {
        result.push((x - 1, y));
    }
    if x < pipes[0].len() - 1 && pipes[y][x + 1] == Pipe::G {
        result.push((x + 1, y));
    }

    result
}

fn is_edge(pipes: &Vec<Vec<Pipe>>, (x, y): (usize, usize)) -> bool {
    x == 0 || x == pipes[0].len() - 1 || y == 0 || y == pipes.len() - 1
}
