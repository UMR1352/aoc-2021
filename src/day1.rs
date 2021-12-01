use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> usize {
    input
        .iter()
        .tuple_windows()
        .filter(|(prev, cur)| prev < cur)
        .count()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> usize {
    input
        .windows(3)
        .map(|xs| xs.iter().sum::<u32>())
        .tuple_windows()
        .filter(|(prev, cur)| prev < cur)
        .count()
}
