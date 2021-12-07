use std::fs;
use itertools::Itertools;

fn align_crabs(numbers: &Vec<i32>, distance: fn(&Vec<i32>, i32) -> i32) -> i32 {
    let &max = numbers.iter().max().unwrap();
    (0..max)
        .map(|i| distance(&numbers, i))
        .min()
        .unwrap()
}

fn distances1(numbers: &Vec<i32>, target: i32) -> i32 {
    numbers.iter()
        .map(|&number| (number - target).abs())
        .sum()
}

fn sequence_sum(n: i32) -> i32 {
    (n + n * n) / 2
}

fn distances2(numbers: &Vec<i32>, target: i32) -> i32 {
    numbers.iter()
        .map(|&number| (number - target).abs())
        .map(|n| sequence_sum(n))
        .sum()
}

pub(crate) fn solve() {
    let numbers = fs::read_to_string("7.txt")
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect_vec();
    println!("{}", align_crabs(&numbers, distances1));
    println!("{}", align_crabs(&numbers, distances2));
}
