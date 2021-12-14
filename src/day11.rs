use std::collections::HashSet;
use std::fs::read_to_string;
use std::hash::Hash;
use itertools::Itertools;

#[derive(Clone)]
struct Board {
    board: Vec<Vec<i32>>,
}

impl Board {
    fn width(&self) -> usize {
        self.board[0].len()
    }

    fn height(&self) -> usize {
        self.board.len()
    }

    fn is_valid_index(&self, x: &i32, y: &i32) -> bool {
        (*x >= 0) & (*x < self.height() as i32) & (*y >= 0) & (*y < self.width() as i32)
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let x = x as i32;
        let y = y as i32;
        let potential = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1), (x - 1, y - 1), (x + 1, y - 1), (x - 1, y + 1), (x + 1, y + 1)];
        potential.iter()
            .filter(|(x, y)| self.is_valid_index(x, y))
            .map(|(x, y)| (*x as usize, *y as usize))
            .collect_vec()
    }

    fn step(&mut self) -> HashSet<(usize, usize)> {
        let mut flashes: HashSet<(usize, usize)> = HashSet::new();
        let mut triggers: Vec<(usize, usize)> = vec![];
        (0..self.height())
            .for_each(|i| (0..self.width())
                .for_each(|j| {
                    self.board[i][j] += 1;
                    if self.board[i][j] > 9 {
                        triggers.push((i, j));
                        flashes.insert((i, j));
                        self.board[i][j] = 0;
                    }
                }));
        while !triggers.is_empty() {
            let (x, y) = triggers.pop().unwrap();
            self.neighbours(x, y)
                .iter()
                .for_each(|(nx, ny)| {
                    if !flashes.contains(&(*nx, *ny)) {
                        triggers.push((*nx, *ny));
                        flashes.insert((*nx, *ny));
                        self.board[*nx][*ny] = 0;
                    }
                });
        }
        println!("{}", flashes.len());
        flashes
    }
}

fn part1(board: &Board) -> i32 {
    let mut board = board.clone();
    (0..100)
        .map(|index| board.step().len() as i32)
        .sum()
}

fn part2(board: &Board) -> i32 {
    let mut board = board.clone();
    let mut counter = 0;
    loop {
        counter += 1;
        let res = board.step();
        if res.len() == 100 {
            return counter;
        }
    }
}

pub(crate) fn solve() {
    let board = Board {
        board: read_to_string("11.txt").unwrap()
            .lines()
            .map(|line| line.chars().map(|x| x.to_string().parse().unwrap()).collect_vec())
            .collect_vec()
    };
    println!("{}", part1(&board));
    println!("{}", part2(&board));
}