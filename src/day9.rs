#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

const NEIGHBOURS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[aoc(day9, part1)]
pub fn part1(map: &[Vec<u32>]) -> u32 {
    map.iter()
        .enumerate()
        .flat_map(|(i, line)| line.iter().enumerate().map(move |(j, h)| ((i, j), *h)))
        .filter(|((y, x), heat)| {
            NEIGHBOURS.iter().all(|(dx, dy)| {
                let i = (*y as isize + *dy) as usize;
                let j = (*x as isize + *dx) as usize;
                map.get(i)
                    .and_then(|l| l.get(j))
                    .map(|h| heat < h)
                    .unwrap_or(true)
            })
        })
        .fold(0, |sum, (_, h)| sum + h + 1)
}

#[aoc(day9, part2)]
pub fn part2(input: &[Vec<u32>]) -> u32 {
    // Thanks cargo-aoc for forcing me to clone
    let mut input = input.to_owned();
    let mut basins = vec![];

    for y in 0..100 {
        for x in 0..100 {
            if input[y][x] < 9 {
                basins.push(basin(&mut input, x, y));
            }
        }
    }

    basins.sort_unstable();

    basins.into_iter().rev().take(3).product()
}

fn basin(map: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    map[y][x] = 9; // Do not revisit this point
    NEIGHBOURS
        .iter()
        .map(|&(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
        .fold(1, |sum, (x, y)| {
            if map.get(y).and_then(|l| l.get(x)).map_or(false, |h| *h < 9) {
                sum + basin(map, x, y)
            } else {
                sum
            }
        })
}
