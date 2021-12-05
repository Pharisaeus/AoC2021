use std::fs;
use itertools::Itertools;

struct Point {
    x: i32,
    y: i32,
}

struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(line: &str) -> Line {
        let line = line.replace(" -> ", ",");
        let numbers = line.split(",")
            .map(|x| x.parse().unwrap())
            .collect_vec();
        Line {
            p1: Point { x: numbers[0], y: numbers[1] },
            p2: Point { x: numbers[2], y: numbers[3] },
        }
    }
    fn is_straight(&self) -> bool {
        return (self.p1.x == self.p2.x) | (self.p1.y == self.p2.y);
    }
    fn delta(&self, start: i32, end: i32) -> (i32, i32) {
        let length = (end - start).abs();
        let delta = if length > 0 { (end - start) / length } else { 0 };
        return (length, delta);
    }
    fn covered_fields(&self) -> Vec<(usize, usize)> {
        let (length_x, delta_x) = self.delta(self.p1.x, self.p2.x);
        let (length_y, delta_y) = self.delta(self.p1.y, self.p2.y);
        return (0..(length_x).max(length_y) + 1)
            .map(|i| ((self.p1.x + delta_x * i) as usize, (self.p1.y + delta_y * i) as usize))
            .collect_vec();
    }
}

struct Board {
    board: Vec<Vec<i32>>,
}

impl Board {
    fn new(x: i32, y: i32) -> Board {
        return Board {
            board: (0..x)
                .map(|_| (0..y)
                    .map(|_| 0)
                    .collect_vec())
                .collect_vec()
        };
    }
    fn count_above_threshold(&self, bound: i32) -> i32 {
        return self.board.iter()
            .flat_map(|row| row.iter())
            .filter(|&&field| field >= bound)
            .count() as i32;
    }

    fn mark_field(&mut self, x: &usize, y: &usize) {
        self.board[*x][*y] += 1;
    }

    fn mark_line(&mut self, line: &Line) {
        line.covered_fields()
            .iter()
            .for_each(|(x, y)| self.mark_field(x, y));
    }
}

fn part1(lines: &Vec<Line>) -> i32 {
    let mut board = Board::new(1000, 1000);
    lines.iter()
        .filter(|line| line.is_straight())
        .for_each(|line| board.mark_line(line));
    board.count_above_threshold(2)
}

fn part2(lines: &Vec<Line>) -> i32 {
    let mut board = Board::new(1000, 1000);
    lines.iter()
        .for_each(|line| board.mark_line(line));
    board.count_above_threshold(2)
}

fn load_data(name: &str) -> Vec<Line> {
    fs::read_to_string(name)
        .unwrap()
        .lines()
        .map(|line| Line::new(line))
        .collect_vec()
}

pub(crate) fn solve() {
    let lines = load_data("5.txt");
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
