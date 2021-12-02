use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(' ')
            .map(|(c, amt)| {
                let amt = amt.parse::<u32>().unwrap();
                match c {
                    "forward" => Ok(Command::Forward(amt)),
                    "down" => Ok(Command::Down(amt)),
                    "up" => Ok(Command::Up(amt)),
                    _ => Err(()),
                }
            })
            .unwrap()
    }
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Command> {
    input.lines().map(|c| c.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Command]) -> u32 {
    let (x, depth) = input
        .iter()
        .fold((0, 0), |(x, depth), command| match command {
            Command::Down(amt) => (x, depth + *amt),
            Command::Up(amt) => (x, depth - *amt),
            Command::Forward(amt) => (x + *amt, depth),
        });

    x * depth
}

#[aoc(day2, part2)]
pub fn part2(input: &[Command]) -> u32 {
    let (x, depth, _aim) = input
        .iter()
        .fold((0, 0, 0), |(x, d, a), command| match command {
            Command::Down(amt) => (x, d, a + *amt),
            Command::Up(amt) => (x, d, a - *amt),
            Command::Forward(amt) => (x + *amt, d + *amt * a, a),
        });

    x * depth
}
