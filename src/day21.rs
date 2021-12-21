use std::collections::HashMap;
use std::thread;

fn part1(p1: i32, p2: i32) -> i32 {
    let mut dice = 1;
    let mut p1 = p1;
    let mut p2 = p2;
    let mut s1 = 0;
    let mut s2 = 0;
    while (s1 < 1000) & (s2 < 1000) {
        let steps = dice * 3 + 3;
        dice = dice + 3;
        p1 = (p1 + steps - 1) % 10 + 1;
        s1 += p1;
        if s1 >= 1000 {
            break;
        }
        let steps = dice * 3 + 3;
        dice = dice + 3;
        p2 = (p2 + steps - 1) % 10 + 1;
        s2 += p2
    }
    return (dice - 1) * s1.min(s2);
}

#[derive(PartialEq, Eq, Hash)]
struct State {
    p1: i32,
    p2: i32,
    s1: i32,
    s2: i32,
    is_p1_turn: bool,
    dices: Vec<i32>,
    current: (i128, i128),
}

fn cached_simulate(state: State, cache: &mut HashMap<State, (i128, i128)>) -> (i128, i128) {
    if let Some(cached) = cache.get(&state) {
        *cached
    } else {
        let res = simulate(&state, cache);
        cache.insert(state, res);
        res
    }
}

fn simulate(state: &State, cache: &mut HashMap<State, (i128, i128)>) -> (i128, i128) {
    let (mut win1, mut win2) = state.current;
    if state.dices.len() < 3 {
        for d in 1..3 {
            let mut new_dices = state.dices.clone();
            new_dices.push(d);
            let new_state = State {
                p1: state.p1,
                p2: state.p2,
                s1: state.s1,
                s2: state.s2,
                is_p1_turn: state.is_p1_turn,
                dices: new_dices,
                current: state.current,
            };
            let (ww1, ww2) = cached_simulate(new_state, cache);
            win1 += ww1;
            win2 += ww2;
        }
        (win1, win2)
    } else {
        let (mut p1, mut p2, mut s1, mut s2) = (state.p1, state.p2, state.s1, state.s2);
        let steps: i32 = state.dices.iter().sum();
        if state.is_p1_turn {
            p1 = (p1 + steps - 1) % 10 + 1;
            s1 += p1
        } else {
            p2 = (p2 + steps - 1) % 10 + 1;
            s2 += p2
        }
        if s1 >= 21 {
            (win1 + 1, win2)
        } else if s2 >= 21 {
            (win1, win2 + 1)
        } else {
            let new_state = State {
                p1,
                p2,
                s1,
                s2,
                is_p1_turn: !state.is_p1_turn,
                dices: vec![],
                current: state.current,
            };
            cached_simulate(new_state, cache)
        }
    }
}

fn part2(p1: i32, p2: i32) -> i128 {
    let state = State {
        p1,
        p2,
        s1: 0,
        s2: 0,
        is_p1_turn: true,
        dices: vec![],
        current: (0, 0),
    };
    let mut cache: HashMap<State, (i128, i128)> = HashMap::new();
    let scores = cached_simulate(state, &mut cache);
    scores.0.max(scores.1)
}

pub(crate) fn solve() {
    println!("{}", part1(4, 8));
    println!("{}", part2(4, 8));
}