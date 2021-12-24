use itertools::Itertools;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|l| {
            l.split_once(" | ")
                .unwrap()
                .1
                .split_ascii_whitespace()
                .map(str::len)
        })
        .filter(|segs_num| matches!(segs_num, 2 | 3 | 4 | 7))
        .count()
}

fn decode(l: &str) -> i32 {
    let (signals, digits) = l.split_once(" | ").unwrap();
    let mut signals = signals.split_ascii_whitespace();
    let one = signals.clone().find(|sig| sig.len() == 2).unwrap();
    let four = signals.find(|sig| sig.len() == 4).unwrap();
    digits
        .split_ascii_whitespace()
        .map(|digit| match digit.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            len => match (
                len,
                digit.chars().filter(|c| one.contains(*c)).count(),
                digit.chars().filter(|c| four.contains(*c)).count(),
            ) {
                (6, 2, 3) => 0,
                (5, 1, 2) => 2,
                (5, 2, 3) => 3,
                (5, 1, 3) => 5,
                (6, 1, 3) => 6,
                (6, 2, 4) => 9,
                _ => unreachable!(),
            },
        })
        .skip_while(|d| *d == 0)
        .fold1(|acc, d| acc * 10 + d)
        .unwrap()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i32 {
    input.lines().map(decode).sum()
}
