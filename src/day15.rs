use std::collections::HashMap;
use std::fs::read_to_string;
use itertools::Itertools;

struct Graph {
    weights: Vec<Vec<i32>>,
    distances: HashMap<(usize, usize), i32>,
    expansion: usize,
}

impl Graph {
    fn new(weights: Vec<Vec<i32>>, expansion: usize) -> Graph {
        let height = weights.len() * expansion;
        let width = weights.first().unwrap().len() * expansion;
        let mut distances: HashMap<(usize, usize), i32> = (0..height)
            .flat_map(|i| (0..width)
                .map(move |j| ((i, j), 99999))
            )
            .collect();
        *distances.get_mut(&(0, 0)).unwrap() = 0;
        Graph {
            weights,
            distances,
            expansion,
        }
    }
    fn height(&self) -> usize {
        return self.weights.len() * self.expansion;
    }

    fn width(&self) -> usize {
        return self.weights.first().unwrap().len() * self.expansion;
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let x = x as i32;
        let y = y as i32;
        let potential = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        potential.iter()
            .filter(|(x, y)| self.is_valid_index(x, y))
            .map(|(x, y)| (*x as usize, *y as usize))
            .collect_vec()
    }

    fn is_valid_index(&self, x: &i32, y: &i32) -> bool {
        (*x >= 0) & (*x < self.height() as i32) & (*y >= 0) & (*y < self.width() as i32)
    }

    fn get_distance_to(&self, x: usize, y: usize) -> i32 {
        return self.distances[&(x, y)];
    }

    fn weight(&self, x: usize, y: usize) -> i32 {
        let base_width = self.width() / self.expansion;
        let base_height = self.height() / self.expansion;
        let base_x = x % base_width;
        let mult_x = x / base_width;
        let base_y = y % base_height;
        let mult_y = y / base_height;
        let w = self.weights[base_x][base_y] + (mult_x as i32) + (mult_y as i32);
        if w > 9 {
            w % 10 + 1
        } else {
            w
        }
    }

    fn compute_distances(&mut self) {
        // simplified bellman-ford
        for _ in 0..4 { // who needs more iterations?
            for x in 0..self.height() {
                for y in 0..self.width() {
                    let node = (x, y);
                    if self.distances[&node] != 99999 {
                        for (nx, ny) in self.neighbours(x, y) {
                            if self.distances[&(nx, ny)] > self.distances[&node] + self.weight(nx, ny) {
                                let new_weight = self.distances[&node] + self.weight(nx, ny);
                                let key = &(nx, ny);
                                *self.distances.get_mut(key).unwrap() = new_weight;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn part1(graph: &mut Graph) -> i32 {
    graph.compute_distances();
    return graph.get_distance_to(graph.width() - 1, graph.height() - 1);
}

pub(crate) fn solve() {
    let weights = read_to_string("15.txt").unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c.to_string().parse().unwrap()).collect_vec())
        .collect_vec();
    let mut g = Graph::new(weights.clone(), 1);
    println!("{}", part1(&mut g));
    let mut g = Graph::new(weights.clone(), 5);
    println!("{}", part1(&mut g));
}
