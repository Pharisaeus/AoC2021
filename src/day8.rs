use std::fs;
use itertools::Itertools;
use std::collections::HashMap;

struct InputSet {
    numbers: Vec<String>,
    outputs: Vec<String>,
}

fn is_valid_digit(v: &str) -> bool {
    ["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"].contains(&v)
}

fn decode_digit(v: &str) -> i32 {
    match v {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => panic!("AAAAAA")
    }
}

fn construct_mapping(permutation: &Vec<char>) -> HashMap<char, char> {
    "abcdefg".chars()
        .zip(permutation.iter())
        .map(|(a, &b)| (a, b))
        .collect()
}

impl InputSet {
    fn new(line: &str) -> InputSet {
        let split = line.split(" | ").collect_vec();
        return InputSet {
            numbers: split[0].split(" ").map(|x| x.to_string()).collect_vec(),
            outputs: split[1].split(" ").map(|x| x.to_string()).collect_vec(),
        };
    }

    fn count_unique_outputs(&self) -> usize {
        self.outputs
            .iter()
            .filter(|x| [2, 3, 4, 7].contains(&(x.len() as i32)))
            .count()
    }

    fn is_valid_mapping(&self, mapping: &HashMap<char, char>) -> bool {
        self.outputs.iter().chain(self.numbers.iter())
            .map(|number| self.apply_mapping(&mapping, &number))
            .all(|n| is_valid_digit(n.as_str()))
    }

    fn apply_mapping(&self, mapping: &HashMap<char, char>, s: &String) -> String {
        s.chars()
            .map(|c| mapping.get(&c).unwrap())
            .sorted()
            .collect()
    }

    fn decode_with_mapping(&self, mapping: &HashMap<char, char>) -> i32 {
        let digits = self.outputs
            .iter()
            .map(|number| self.apply_mapping(&mapping, number))
            .map(|x| decode_digit(x.as_str()))
            .collect_vec();
        digits.iter()
            .fold(0, |acc, digit| acc * 10 + digit)
    }

    fn decode_output(&self) -> i32 {
        "abcdefg".chars()
            .permutations(7)
            .map(|perm| construct_mapping(&perm))
            .filter(|mapping| self.is_valid_mapping(mapping))
            .map(|mapping| self.decode_with_mapping(&mapping))
            .next()
            .unwrap()
    }
}

fn part1(data: &Vec<InputSet>) -> usize {
    data.iter()
        .map(|x| x.count_unique_outputs())
        .sum()
}

fn part2(data: &Vec<InputSet>) -> i32 {
    data.iter()
        .map(|x| x.decode_output())
        .sum()
}

pub(crate) fn solve() {
    let inputs = fs::read_to_string("8.txt")
        .unwrap()
        .lines()
        .map(|line| InputSet::new(line))
        .collect_vec();
    println!("{}", part1(&inputs));
    println!("{}", part2(&inputs));
}

