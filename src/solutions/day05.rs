use super::Solver;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::str::FromStr;

pub struct Problem;

#[derive(Debug)]
struct Mapping {
    source: usize,
    destination: usize,
    range: usize,
}

impl FromStr for Mapping {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (destination, source, range) = s
            .split(" ")
            .collect_tuple()
            .ok_or("Unkown mapping format")?;

        return Ok(Mapping {
            source: source.parse().unwrap(),
            destination: destination.parse().unwrap(),
            range: range.parse().unwrap(),
        });
    }
}

impl Mapping {
    fn map(self: &Self, v: usize) -> Option<usize> {
        if v >= self.source && v < self.source + self.range {
            Some(self.destination + v - self.source)
        } else {
            None
        }
    }

    fn map_range(self: &Self, range: &Range<usize>) -> (Option<Range<usize>>, Vec<Range<usize>>) {
        let self_end = self.source + self.range;
        let start = range.start.max(self.source);
        let end = range.end.min(self_end);

        if end > start {
            let mut unmapped = Vec::new();
            if start > range.start {
                unmapped.push(Range {
                    start: range.start,
                    end: start,
                })
            }
            if end < range.end {
                unmapped.push(Range {
                    start: end,
                    end: range.end,
                })
            }
            (
                Some(Range {
                    start: self.destination + start - self.source,
                    end: self.destination + end - self.source,
                }),
                unmapped,
            )
        } else {
            (None, vec![range.to_owned()])
        }
    }
}

struct Mappings {
    mappings: Vec<Mapping>,
}
impl Mappings {
    fn map_seed(self: &Self, seed: usize) -> usize {
        self.mappings
            .iter()
            .find_map(|mapping| mapping.map(seed))
            .unwrap_or(seed)
    }

    fn map_range(self: &Self, range: &Range<usize>) -> Vec<Range<usize>> {
        let mut unmapped_ranges = vec![range.to_owned()];
        let mut mapped_ranges = Vec::new();

        for mapping in &self.mappings {
            let mut new_unmapped_ranges = Vec::new();
            for range in unmapped_ranges {
                let (mapped, unmapped) = mapping.map_range(&range);
                new_unmapped_ranges.extend(unmapped);
                if let Some(mapped) = mapped {
                    mapped_ranges.push(mapped);
                }
            }
            unmapped_ranges = new_unmapped_ranges;
        }

        mapped_ranges.append(&mut unmapped_ranges);

        merge_ranges(&mapped_ranges)
    }
}

pub struct Almanac {
    seeds: Vec<usize>,
    seed_ranges: Vec<Range<usize>>,
    mappings: Vec<Mappings>,
}

impl Almanac {
    fn get_seed_location(self: &Self, seed: usize) -> usize {
        self.mappings
            .iter()
            .fold(seed, |acc, mappings| mappings.map_seed(acc))
    }

    fn get_seed_range_location(self: &Self, range: &Range<usize>) -> Vec<Range<usize>> {
        self.mappings
            .iter()
            .fold(vec![range.to_owned()], |acc, mappings| {
                acc.iter().flat_map(|r| mappings.map_range(r)).collect()
            })
    }
}

impl Solver for Problem {
    type Input = Almanac;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();

        let (_, seed_list) = lines[0].split(": ").collect_tuple().unwrap();
        let seeds = seed_list
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect_vec();
        let seed_ranges = seeds.chunks(2).map(|v| v[0]..(v[0] + v[1])).collect_vec();

        let mut mappings = Vec::new();
        let mut current_mapping = Vec::new();
        for i in 3..lines.len() {
            if lines[i].contains("map") {
                mappings.push(Mappings {
                    mappings: current_mapping,
                });
                current_mapping = Vec::new();
            } else if lines[i] != "" {
                current_mapping.push(lines[i].parse().unwrap());
            }
        }
        mappings.push(Mappings {
            mappings: current_mapping,
        });

        Almanac {
            seeds,
            mappings,
            seed_ranges,
        }
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input
            .seeds
            .iter()
            .map(|seed| input.get_seed_location(*seed))
            .min()
            .unwrap())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        Ok(input
            .seed_ranges
            .iter()
            .map(|seed_range| input.get_seed_range_location(seed_range))
            .map(|ranges| ranges.iter().map(|range| range.start).min().unwrap())
            .min()
            .unwrap())
    }
}

fn merge_ranges(ranges: &Vec<Range<usize>>) -> Vec<Range<usize>> {
    let mut result = Vec::new();

    let sorted = ranges
        .iter()
        .sorted_by(|a, b| a.start.cmp(&b.start))
        .collect_vec();

    for range in sorted {
        if result.len() == 0 {
            result.push(range.clone());
            continue;
        }

        let i = result.len() - 1;
        if result[i].end >= range.start {
            result[i].end = result[i].end.max(range.end);
        } else {
            result.push(range.clone());
        }
    }

    result
}
