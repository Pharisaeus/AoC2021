use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::ops::Add;
use itertools::Itertools;

#[derive(Clone)]
struct Fold {
    axis: String,
    pos: i32,
}

impl Fold {
    fn create(line: &str) -> Fold {
        let x = line.split("=").collect_vec();
        Fold {
            axis: x[0].split(" ").collect_vec().last().unwrap().to_string(),
            pos: x[1].parse().unwrap(),
        }
    }
}

struct Board {
    folds: Vec<Fold>,
    coords: HashSet<(i32, i32)>,
}

impl Board {
    fn dots(&self) -> usize {
        self.coords.len()
    }

    fn step(&self) -> Board {
        let f = self.folds.first().unwrap();
        let mut new_coords = HashSet::new();
        if f.axis == "y" {
            for &(x, y) in &self.coords {
                if y < f.pos {
                    new_coords.insert((x, y));
                } else if y != f.pos {
                    new_coords.insert((x, 2 * f.pos - y));
                }
            }
        } else {
            for &(x, y) in &self.coords {
                if x < f.pos {
                    new_coords.insert((x, y));
                } else if x != f.pos {
                    new_coords.insert((2 * f.pos - x, y));
                }
            }
        }
        Board {
            folds: self.folds.iter().skip(1).map(|x| x.clone()).collect_vec(),
            coords: new_coords,
        }
    }

    fn to_str(&self) -> String {
        let my = self.coords.iter().map(|(x, y)| y).max().unwrap();
        let mx = self.coords.iter().map(|(x, y)| x).max().unwrap();
        let mut res = mx.to_string().add(" ").add(my.to_string().as_str()).add("\n");
        for i in 0..(my + 1) {
            for j in 0..(mx + 1) {
                if self.coords.contains(&(j, i)) {
                    res = res.add("#");
                } else {
                    res = res.add(" ");
                }
            }
            res = res.add("\n");
        }
        res
    }
}


fn part1(board: &Board) -> usize {
    board.step().dots()
}

fn part2(board: &Board) -> String {
    let mut b = board.step();
    while b.folds.len() > 0 {
        b = b.step();
    }
    b.to_str()
}


pub(crate) fn solve() {
    let data = read_to_string("13.txt").unwrap();
    let x = data.split("\r\n\r\n").collect_vec();
    let coords = x[0];
    let splits = x[1];
    let board = Board {
        folds: splits.lines()
            .map(|line| Fold::create(line))
            .collect_vec(),
        coords: coords.lines()
            .map(|line|
                line.split(",")
                    .collect_vec())
            .map(|x| (x[0].parse().unwrap(), x[1].parse().unwrap()))
            .collect(),
    };
    println!("{}", part1(&board));
    println!("{}", part2(&board));
}
