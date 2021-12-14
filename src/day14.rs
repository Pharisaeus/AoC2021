use std::collections::HashMap;
use std::fs::read_to_string;
use itertools::Itertools;

struct DefaultDict {
    d: HashMap<String, i64>,
}

impl DefaultDict {
    fn get(&mut self, key: &String) -> i64 {
        if self.d.contains_key(key) {
            *self.d.get(key).unwrap()
        } else {
            self.d.insert(key.to_string(), 0);
            return 0;
        }
    }
    fn bump(&mut self, key: &String, count: i64) {
        let current = self.get(key);
        self.d.insert(key.to_string(), current + count);
    }
}

fn part1(start: &String, mapping: &HashMap<String, String>) -> usize {
    let mut current = start.clone();
    for _ in 0..10 {
        let mut new_value = "".to_string();
        for (a, b) in current.chars().zip(current.chars().skip(1)) {
            new_value += a.to_string().as_str();
            let key = (a.to_string() + b.to_string().as_str());
            new_value += mapping.get(key.as_str()).unwrap_or(&"".to_string());
        }
        new_value += current.chars().last().unwrap().to_string().as_str();
        current = new_value;
    }
    let x = current.chars().counts_by(|x| x);
    let high = x.iter().max_by_key(|&(c, count)| count).unwrap();
    let low = x.iter().min_by_key(|&(c, count)| count).unwrap();
    high.1 - low.1
}

fn part2(start: &String, mapping: &HashMap<String, String>) -> i64 {
    let mut current = start.clone();
    let mut occurrences = DefaultDict { d: HashMap::new() };
    for (a, b) in current.chars().zip(current.chars().skip(1)) {
        let key = (a.to_string() + b.to_string().as_str());
        occurrences.bump(&key, 1);
    }
    for _ in 0..40 {
        let mut new_occurrences = DefaultDict { d: HashMap::new() };
        for (key, count) in &(occurrences.d) {
            if mapping.contains_key(key.as_str()) {
                let prefix_key = key.chars().collect_vec()[0].to_string() + mapping.get(key.as_str()).unwrap();
                let suffix_key = mapping.get(key.as_str()).unwrap().to_string() + key.chars().collect_vec()[1].to_string().as_str();
                new_occurrences.bump(&prefix_key, *count);
                new_occurrences.bump(&suffix_key, *count);
            } else {
                new_occurrences.bump(key, *count);
            }
        }
        occurrences = new_occurrences;
    }
    let mut elem_count = DefaultDict { d: HashMap::new() };
    for (pair, count) in &(occurrences.d) {
        let key = pair.chars().collect_vec()[0].to_string();
        elem_count.bump(&key, *count);
    }
    let key = start.chars().last().unwrap().to_string();
    elem_count.bump(&key, 1);
    elem_count.d.values().max().unwrap() - elem_count.d.values().min().unwrap()
}

pub(crate) fn solve() {
    let data = read_to_string("14.txt").unwrap();
    let x = data.split("\r\n\r\n").collect_vec();
    let start = x[0].to_string();
    let subs = x[1];
    let mapping: HashMap<String, String> = subs.lines()
        .map(|line| line.split(" -> ").collect_vec())
        .map(|x| (x[0].to_string(), x[1].to_string()))
        .collect();
    println!("{}", part1(&start, &mapping));
    println!("{}", part2(&start, &mapping));
}
