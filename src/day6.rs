use std::fs;
use itertools::Itertools;
use std::collections::HashMap;

fn add(mut fish: HashMap<i32, i64>, key: i32, val: i64) -> HashMap<i32, i64> {
    fish.insert(key, fish.get(&key).unwrap_or(&0) + val);
    return fish;
}

fn part1(numbers: &Vec<i32>, rounds: i32) -> i64 {
    let mut fish: HashMap<i32, i64> = HashMap::new();
    for &number in numbers {
        fish = add(fish, number, 1);
    }
    for _ in 0..rounds {
        let mut new_fish: HashMap<i32, i64> = HashMap::new();
        for (days, count) in fish {
            match days {
                0 => {
                    new_fish = add(new_fish, 6, count);
                    new_fish = add(new_fish, 8, count);
                }
                _ => {
                    new_fish = add(new_fish, days - 1, count);
                }
            }
        }
        fish = new_fish
    }
    return fish.iter().map(|(_, count)| count).sum();
}

pub(crate) fn solve() {
    let numbers = fs::read_to_string("6.txt")
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect_vec();
    println!("{}", part1(&numbers, 80));
    println!("{}", part1(&numbers, 256));
}
