use std::borrow::Borrow;
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
        return match &line[..1] {
            "f" => Command::Forward { val },
            "u" => Command::Up { val },
            "d" => Command::Down { val },
            _ => Command::Invalid
        };
    }
}


fn part1(commands: &[Command]) -> i32 {
    let mut pos = 0;
    let mut depth = 0;
    for command in commands.iter() {
        match command {
            Command::Forward { val } => pos += val,
            Command::Down { val } => depth += val,
            Command::Up { val } => depth -= val,
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
        match command {
            Command::Forward { val } => {
                pos += val;
                depth += val * aim
            }
            Command::Down { val } => aim += val,
            Command::Up { val } => aim -= val,
            _ => {}
        }
    };
    return pos * depth;
}

// more functional approach below:

struct State1 {
    pos: i32,
    depth: i32,
}

fn move_sub1(state: State1, command: &Command) -> State1 {
    return match command {
        Command::Forward { val } => State1 { pos: state.pos + val, depth: state.depth },
        Command::Down { val } => State1 { pos: state.pos, depth: state.depth + val },
        Command::Up { val } => State1 { pos: state.pos, depth: state.depth - val },
        _ => state
    };
}

fn part1_fun(commands: &[Command]) -> i32 {
    let final_state = commands.iter()
        .fold(State1 { pos: 0, depth: 0 }, move_sub1);
    return final_state.pos * final_state.depth;
}

struct State2 {
    pos: i32,
    depth: i32,
    aim: i32,
}

fn move_sub2(state: State2, command: &Command) -> State2 {
    return match command {
        Command::Forward { val } => State2 { pos: state.pos + val, depth: state.depth + state.aim * val, aim: state.aim },
        Command::Down { val } => State2 { pos: state.pos, depth: state.depth, aim: state.aim + val },
        Command::Up { val } => State2 { pos: state.pos, depth: state.depth, aim: state.aim - val },
        _ => state
    };
}

fn part2_fun(commands: &[Command]) -> i32 {
    let final_state = commands.iter()
        .fold(State2 { pos: 0, depth: 0, aim: 0 }, move_sub2);
    return final_state.pos * final_state.depth;
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("2.txt").unwrap();
    let commands: Vec<Command> = contents.lines()
        .map(|line| Command::new(line))
        .collect();
    println!("{}", part1(&commands));
    println!("{}", part1_fun(&commands));
    println!("{}", part2(&commands));
    println!("{}", part2_fun(&commands));
}
