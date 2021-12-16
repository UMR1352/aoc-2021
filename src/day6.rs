#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let mut fish_count =
        input
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .fold(vec![0; 9], |mut acc, x| {
                acc[x] += 1;
                acc
            });

    (1..=80).for_each(|_| {
        let ready_to_spawn = fish_count.remove(0);
        fish_count.push(ready_to_spawn);
        fish_count[6] += fish_count[8];
    });

    fish_count.iter().sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut fish_count =
        input
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .fold(vec![0; 9], |mut acc, x| {
                acc[x] += 1;
                acc
            });

    (0..256).for_each(|_| {
        let ready_to_spawn = fish_count.remove(0);
        fish_count.push(ready_to_spawn);
        fish_count[6] += fish_count[8];
    });

    fish_count.iter().sum()
}
