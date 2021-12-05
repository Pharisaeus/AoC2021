use std::fs;
use itertools::Itertools;

enum Command {
    Forward { val: i32 },
    Up { val: i32 },
    Down { val: i32 },
    Invalid,
}

impl Command {
    pub fn new(line: &str) -> Self {
        let val = line.split(" ").collect_vec()[1].parse().unwrap();
        match &line[..1] {
            "f" => Command::Forward { val },
            "u" => Command::Up { val },
            "d" => Command::Down { val },
            _ => Command::Invalid
        }
    }
}

#[derive(Default)]
struct State1 {
    pos: i32,
    depth: i32,
}

impl State1 {
    fn score(&self) -> i32 {
        self.pos * self.depth
    }
}

fn move_sub1(state: State1, command: &Command) -> State1 {
    match command {
        Command::Forward { val } => State1 { pos: state.pos + val, ..state },
        Command::Down { val } => State1 { depth: state.depth + val, ..state },
        Command::Up { val } => State1 { depth: state.depth - val, ..state },
        _ => state
    }
}

fn part1(commands: &[Command]) -> i32 {
    commands.iter()
        .fold(Default::default(), move_sub1)
        .score()
}

#[derive(Default)]
struct State2 {
    pos: i32,
    depth: i32,
    aim: i32,
}

impl State2 {
    fn score(&self) -> i32 {
        self.pos * self.depth
    }
}

fn move_sub2(state: State2, command: &Command) -> State2 {
    match command {
        Command::Forward { val } => State2 { pos: state.pos + val, depth: state.depth + state.aim * val, ..state },
        Command::Down { val } => State2 { aim: state.aim + val, ..state },
        Command::Up { val } => State2 { aim: state.aim - val, ..state },
        _ => state
    }
}

fn part2(commands: &[Command]) -> i32 {
    commands.iter()
        .fold(Default::default(), move_sub2)
        .score()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("2.txt").unwrap();
    let commands = contents.lines()
        .map(|line| Command::new(line))
        .collect_vec();
    println!("{}", part1(&commands));
    println!("{}", part2(&commands));
}
