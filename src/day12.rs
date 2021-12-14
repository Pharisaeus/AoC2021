use std::collections::HashMap;
use std::fs::read_to_string;
use itertools::{all, Itertools};

struct Graph {
    edges: HashMap<String, Vec<String>>,
}

impl Graph {
    fn neighbours(&self, src: &String) -> Vec<String> {
        match self.edges.get(src) {
            None => vec![],
            Some(x) => x.clone()
        }
    }
}

fn find_paths(graph: &Graph, current_path: Vec<String>, double_visit: bool, all_paths: &mut Vec<Vec<String>>) -> () {
    let current_node = current_path.last().unwrap();
    for neighbour in graph.neighbours(current_node) {
        let mut new_path = current_path.clone();
        new_path.push(neighbour.to_string());
        if neighbour == "end" {
            all_paths.push(new_path);
        } else if (neighbour == neighbour.to_uppercase()) | !current_path.contains(&neighbour) {
            find_paths(graph, new_path, double_visit, all_paths);
        } else if double_visit & (neighbour != "start") {
            find_paths(graph, new_path, false, all_paths)
        }
    }
}

fn part1(graph: &Graph) -> usize {
    let mut results: Vec<Vec<String>> = vec![];
    find_paths(graph, vec!["start".to_string()], false, &mut results);
    results.len()
}

fn part2(graph: &Graph) -> usize {
    let mut results: Vec<Vec<String>> = vec![];
    find_paths(graph, vec!["start".to_string()], true, &mut results);
    results.len()
}

pub(crate) fn solve() {
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    read_to_string("12.txt").unwrap()
        .lines()
        .map(|line| line.split("-").collect_vec())
        .for_each(|t| {
            let neighbours = edges.entry(t[0].to_string()).or_insert(vec![]);
            neighbours.push(t[1].to_string());
            let neighbours = edges.entry(t[1].to_string()).or_insert(vec![]);
            neighbours.push(t[0].to_string())
        });
    let graph = Graph {
        edges
    };
    println!("{}", part1(&graph));
    println!("{}", part2(&graph));
}