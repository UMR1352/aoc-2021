#[aoc(day7, part1)]
pub fn part1(input: &str) -> i32 {
    let mut subs = input
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    let mid = subs.len() / 2;
    let median = *subs.select_nth_unstable(mid).1;
    subs.iter().map(|x| (x - median).abs()).sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> i32 {
    let subs = input
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    let mean = subs.iter().sum::<i32>() / subs.len() as i32;

    subs.iter()
        .map(|s| (mean - *s).abs())
        .map(|d| d * (d + 1) / 2)
        .sum()
}
