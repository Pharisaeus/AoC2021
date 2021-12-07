use std::fs;
use itertools::Itertools;

fn bf_align_crabs(numbers: &Vec<i32>, distance: fn(&Vec<i32>, i32) -> i32) -> i32 {
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

fn part1(numbers: &Vec<i32>) -> i32 {
    bf_align_crabs(numbers, distances1)
}

fn part1_faster(numbers: &Vec<i32>) -> i32 {
    let mut sorted = numbers.clone();
    sorted.sort();
    let median = sorted[sorted.len() / 2];
    distances1(numbers, median)
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

fn part2(numbers: &Vec<i32>) -> i32 {
    bf_align_crabs(numbers, distances2)
}

fn part2_faster(numbers: &Vec<i32>) -> i32 {
    let avg: i32 = numbers.iter().sum::<i32>() / numbers.len() as i32;
    (avg..avg + 1)
        .map(|i| distances2(&numbers, i))
        .min()
        .unwrap()
}

pub(crate) fn solve() {
    let numbers = fs::read_to_string("7.txt")
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect_vec();
    println!("{}", part1(&numbers));
    println!("{}", part1_faster(&numbers));
    println!("{}", part2(&numbers));
    println!("{}", part2_faster(&numbers));
}
