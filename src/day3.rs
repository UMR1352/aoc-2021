#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut gamma = 0;
    let mut epsilon = 0;
    for pos in 0..lines[0].len() {
        gamma <<= 1;
        epsilon <<= 1;
        let ones_count = lines
            .iter()
            .map(|x| x.chars().nth(pos).unwrap())
            .filter(|x| *x == '1')
            .count();
        if ones_count >= (lines.len() - ones_count) {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    part2_solver(&lines, true) * part2_solver(&lines, false)
}

fn part2_solver(lines: &[&str], most: bool) -> u32 {
    let mut pos = 0;
    let mut values = lines.to_owned();
    while values.len() > 1 {
        let ones_count = values
            .iter()
            .map(|x| x.chars().nth(pos).unwrap())
            .filter(|x| *x == '1')
            .count();
        if ones_count >= (values.len() - ones_count) {
            values.retain(|x| x.chars().nth(pos).unwrap() == if most { '1' } else { '0' })
        } else {
            values.retain(|x| x.chars().nth(pos).unwrap() == if most { '0' } else { '1' })
        }
        pos += 1
    }
    u32::from_str_radix(values[0], 2).unwrap()
}
