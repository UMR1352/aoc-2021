use std::collections::HashMap;

const ROW: u32 = 0b11111;
const COL: u32 = 0b100001000010000100001;

pub type Board = HashMap<u8, usize>;

#[aoc_generator(day4)]
pub fn generator(input: &str) -> (Vec<u8>, Vec<(Board, u32)>) {
    let (nums, boards) = input.split_once("\n\n").unwrap();

    let nums = nums.split(',').map(|x| x.parse().unwrap()).collect();
    let boards = boards
        .split("\n\n")
        .map(|b| {
            (
                b.split_ascii_whitespace()
                    .enumerate()
                    .map(|(i, x)| (x.parse().unwrap(), i))
                    .collect(),
                0,
            )
        })
        .collect();

    (nums, boards)
}

#[aoc(day4, part1)]
pub fn part1(input: &(Vec<u8>, Vec<(Board, u32)>)) -> u32 {
    // Since cargo-aoc doens't let me move the generetor's output I have to clone...
    let (nums, mut boards) = (&input.0, input.1.clone());

    let (board, mark, num) = nums
        .iter()
        .find_map(|num| {
            boards.iter_mut().find_map(|(board, mark)| {
                board
                    .get(num)
                    .map(|i| *mark |= 1 << *i)
                    .filter(|_| {
                        (0..5).any(|i| *mark >> i & COL == COL || *mark >> (i * 5) & ROW == ROW)
                    })
                    .map(|_| (board.clone(), *mark, num))
            })
        })
        .unwrap();

    board
        .iter()
        .map(|(x, i)| (mark >> i & 1 ^ 1) * *x as u32 * *num as u32)
        .sum::<u32>()
}

#[aoc(day4, part2)]
pub fn part2(input: &(Vec<u8>, Vec<(Board, u32)>)) -> u32 {
    // Since cargo-aoc doens't let me move the generetor's output I have to clone...
    let (nums, mut boards) = (&input.0, input.1.clone());
    let mut winning_boards = Vec::with_capacity(nums.len() / 2);

    for num in nums {
        let mut i = 0;
        while i < boards.len() {
            let (board, mark) = &mut boards[i];
            if let Some(pos) = board.get(num) {
                *mark |= 1 << *pos;
                if (0..5).any(|i| *mark >> i & COL == COL || *mark >> (i * 5) & ROW == ROW) {
                    winning_boards.push((std::mem::take(board), *mark, *num));
                    boards.swap_remove(i);
                    continue;
                }
            }
            i += 1;
        }
    }

    let (board, mark, num) = winning_boards.last().unwrap();

    board
        .iter()
        .map(|(x, i)| (mark >> i & 1 ^ 1) * *x as u32 * *num as u32)
        .sum::<u32>()
}
