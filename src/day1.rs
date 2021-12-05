use std::fs;

fn part1(data: &[i32]) -> usize {
    let pairs = data.iter().zip(data.iter().skip(1));
    pairs
        .filter(|(x, y)| x < y)
        .count()
}

fn part2(data: &[i32]) -> usize {
    let sum_of_three: Vec<i32> = data
        .windows(3)
        .map(|chunk| chunk.iter().sum())
        .collect();
    part1(&sum_of_three)
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("1.txt").unwrap();
    let numbers: Vec<i32> = contents.split("\r\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    println!("{}", part1(&numbers));
    println!("{}", part2(&numbers));
}
