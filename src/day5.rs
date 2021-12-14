use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

impl Line {
    pub fn is_straight(&self) -> bool {
        self.x0 == self.x1 || self.y0 == self.y1
    }
}

impl IntoIterator for Line {
    type Item = (i32, i32);
    type IntoIter = LineIter;
    fn into_iter(self) -> Self::IntoIter {
        LineIter::new(&self)
    }
}

#[derive(Debug)]
pub struct LineIter {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    err: i32,
    done: bool,
    dx: i32,
    dy: i32,
    sx: i32,
    sy: i32,
}

impl LineIter {
    pub fn new(line: &Line) -> Self {
        let dx = i32::abs(line.x1 - line.x0);
        let dy = -i32::abs(line.y1 - line.y0);
        Self {
            x0: line.x0,
            y0: line.y0,
            x1: line.x1,
            y1: line.y1,
            done: false,
            sx: if line.x0 < line.x1 { 1 } else { -1 },
            sy: if line.y0 < line.y1 { 1 } else { -1 },
            err: dx + dy,
            dx,
            dy,
        }
    }
}

impl Iterator for LineIter {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.x0 == self.x1 && self.y0 == self.y1 {
            if self.done {
                None
            } else {
                self.done = true;
                Some((self.x0, self.y0))
            }
        } else {
            let res = (self.x0, self.y0);
            let e2 = self.err * 2;
            if e2 >= self.dy {
                self.err += self.dy;
                self.x0 += self.sx;
            }
            if e2 <= self.dx {
                self.err += self.dx;
                self.y0 += self.sy;
            }

            Some(res)
        }
    }
}

trait Frequency: Iterator {
    fn frequencies(&mut self) -> HashMap<Self::Item, usize>;
}

impl<T> Frequency for T
where
    T: Iterator,
    T::Item: std::hash::Hash + Eq,
{
    fn frequencies(&mut self) -> HashMap<Self::Item, usize> {
        self.fold(HashMap::new(), |mut freqs, x| {
            let count = freqs.entry(x).or_insert(0);
            *count += 1;

            freqs
        })
    }
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let (p, q) = line.split_once(" -> ").unwrap();
            let (x0, y0) = p.split_once(',').unwrap();
            let (x1, y1) = q.split_once(',').unwrap();
            let x0 = x0.parse().unwrap();
            let y0 = y0.parse().unwrap();
            let x1 = x1.parse().unwrap();
            let y1 = y1.parse().unwrap();

            Line { x0, y0, x1, y1 }
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &[Line]) -> usize {
    input
        .iter()
        .filter(|l| l.is_straight())
        .flat_map(|l| l.into_iter())
        .frequencies()
        .values()
        .filter(|freq| **freq >= 2)
        .count()
}

#[aoc(day5, part2)]
pub fn part2(input: &[Line]) -> usize {
    input
        .iter()
        .flat_map(|l| l.into_iter())
        .frequencies()
        .values()
        .filter(|freq| **freq >= 2)
        .count()
}
