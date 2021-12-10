use std::fs::read_to_string;
use itertools::Itertools;

struct Line {
    line: Vec<String>,
}

impl Line {
    fn new(line: &str) -> Line {
        Line {
            line: line.chars()
                .map(|c| c.to_string())
                .collect_vec()
        }
    }
    fn find_corruption(&self) -> (Option<usize>, Vec<String>) {
        let mut stack: Vec<String> = vec![];
        for (index, value) in self.line.iter().enumerate() {
            if ["(", "[", "{", "<"].contains(&value.as_str()) {
                stack.push(value.clone());
            } else if stack.last().filter(|x| x.as_str() == match value.as_str() {
                ")" => "(",
                "]" => "[",
                "}" => "{",
                ">" => "<",
                _ => ""
            }).is_some() {
                stack.pop();
            } else {
                return (Option::from(index), stack);
            }
        }
        return (Option::None, stack);
    }

    fn is_corrupted(&self) -> bool {
        self.find_corruption().0.is_some()
    }

    fn get_corrupted_score(&self) -> i32 {
        match self.line[self.find_corruption().0.unwrap()].as_str() {
            ")" => 3,
            "]" => 57,
            "}" => 1197,
            ">" => 25137,
            _ => panic!()
        }
    }
    fn get_complete_score(&self) -> i64 {
        self.find_corruption().1
            .iter()
            .rev()
            .map(|symbol| match symbol.as_str() {
                "(" => 1,
                "[" => 2,
                "{" => 3,
                "<" => 4,
                _ => panic!()
            })
            .fold(0, |acc, x| acc * 5 + x)
    }
}

fn part1(lines: &Vec<Line>) -> i32 {
    lines
        .iter()
        .filter(|line| line.is_corrupted())
        .map(|line| line.get_corrupted_score())
        .sum()
}

fn part2(lines: &Vec<Line>) -> i64 {
    let completion_scores = lines
        .iter()
        .filter(|line| !line.is_corrupted())
        .map(|line| line.get_complete_score())
        .sorted()
        .collect_vec();
    completion_scores[completion_scores.len() / 2]
}

pub(crate) fn solve() {
    let data = read_to_string("10.txt").unwrap()
        .lines()
        .map(|line| Line::new(line))
        .collect_vec();
    println!("{}", part1(&data));
    println!("{}", part2(&data));
}