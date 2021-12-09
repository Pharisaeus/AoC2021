use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

struct Board {
    array: Vec<Vec<i32>>,
}

impl Board {
    fn width(&self) -> usize {
        self.array[0].len()
    }

    fn height(&self) -> usize {
        self.array.len()
    }

    fn get_value(&self, x: &usize, y: &usize) -> i32 {
        self.array[*x][*y]
    }

    fn get_risk_level(&self, x: &usize, y: &usize) -> i32 {
        self.get_value(x, y) + 1
    }

    fn is_valid_index(&self, x: &i32, y: &i32) -> bool {
        (*x >= 0) & (*x < self.height() as i32) & (*y >= 0) & (*y < self.width() as i32)
    }

    fn get_neighbours(&self, x: &usize, y: &usize) -> Vec<(usize, usize)> {
        let x = *x as i32;
        let y = *y as i32;
        let potential = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        potential.iter()
            .filter(|(x, y)| self.is_valid_index(x, y))
            .map(|(x, y)| (*x as usize, *y as usize))
            .collect_vec()
    }

    fn is_low_point(&self, x: &usize, y: &usize) -> bool {
        let field = self.get_value(x, y);
        self.get_neighbours(x, y)
            .iter()
            .all(|(nx, ny)| field < self.get_value(nx, ny))
    }

    fn get_low_points(&self) -> Vec<(usize, usize)> {
        (0..self.height())
            .flat_map(|i| (0..self.width())
                .map(move |j| (i, j)))
            .filter(|(i, j)| self.is_low_point(i, j))
            .collect_vec()
    }

    fn get_basin_for_low(&self, start_x: &usize, start_y: &usize) -> HashSet<(usize, usize)> {
        let mut basin = HashSet::new();
        let mut to_check = vec![(*start_x, *start_y)];
        while !to_check.is_empty() {
            let (x, y) = to_check.pop().unwrap();
            basin.insert((x, y));
            self.get_neighbours(&x, &y)
                .iter()
                .filter(|(nx, ny)| !basin.contains(&(*nx, *ny)))
                .filter(|(nx, ny)| self.get_value(nx, ny) != 9)
                .for_each(|(nx, ny)| to_check.push((*nx, *ny)))
        }
        basin
    }
}

fn part1(board: &Board) -> i32 {
    board.get_low_points()
        .iter()
        .map(|(x, y)| board.get_risk_level(x, y))
        .sum()
}

fn part2(board: &Board) -> i32 {
    let basin_sizes = board.get_low_points()
        .iter()
        .map(|(x, y)| board.get_basin_for_low(x, y))
        .map(|basin| basin.len() as i32)
        .sorted_by(|a, b| b.cmp(a))
        .collect_vec();
    basin_sizes[0..3].iter().fold(1, |acc, x| acc * x)
}

pub(crate) fn solve() {
    let inputs = fs::read_to_string("9.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect_vec())
        .collect_vec();
    let board = Board { array: inputs };
    println!("{}", part1(&board));
    println!("{}", part2(&board));
}
