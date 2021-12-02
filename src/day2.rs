use std::borrow::Borrow;
use std::fs;
use itertools::Itertools;

struct Command {
    cmd: String,
    value: i32,
}

impl Command {
    pub fn new(line: &str) -> Self {
        let split = line.split(" ").collect_vec();
        let cmd = split[0].to_string();
        let value = split[1].parse().unwrap();
        Command { cmd, value }
    }
}


fn part1(commands: &[Command]) -> i32 {
    let mut pos = 0;
    let mut depth = 0;
    for command in commands.iter() {
        let value = command.value;
        match command.cmd.as_str() {
            "forward" => pos += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => {}
        }
    };
    return pos * depth;
}

fn part2(commands: &[Command]) -> i32 {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands.iter() {
        let value = command.value;
        match command.cmd.as_str() {
            "forward" => {pos += value; depth += value*aim},
            "down" => aim += value,
            "up" => aim -= value,
            _ => {}
        }
    };
    return pos * depth;
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("2.txt").unwrap();
    let commands: Vec<Command> = contents.lines()
        .map(|line| Command::new(line))
        .collect();
    println!("{}", part1(&commands));
    println!("{}", part2(&commands));
}
