use std::fs::read_to_string;
use itertools::Itertools;

#[derive(Clone)]
struct Command {
    is_on: bool,
    cube: Cube,
}

#[derive(Clone)]
struct Cube {
    axes: Vec<(i128, i128)>,
}

impl Cube {
    fn size(&self) -> i128 {
        self.axes.iter()
            .map(|(start, end)| end - start + 1)
            .product()
    }
    fn find_overlap(&self, another_cube: &Cube) -> Option<Vec<(i128, i128)>> {
        let mut overlapping_regions = Vec::new();
        for i in 0..self.axes.len() {
            let (start1, end1) = self.axes[i];
            let (start2, end2) = another_cube.axes[i];
            if ((start1 <= end2) & (end2 <= end1)) | ((start2 <= end1) & (end1 <= end2)) {
                overlapping_regions.push((start1.max(start2), end1.min(end2)));
            } else {
                return None;
            }
        }
        Some(overlapping_regions)
    }
    fn split_on_overlap(&self, overlap: Vec<(i128, i128)>) -> Vec<Cube> {
        let mut new_cubes = Vec::new();
        for i in 0..self.axes.len() {
            let (overlap_start, overlap_end) = overlap[i];
            let (my_start, my_end) = self.axes[i];
            if overlap_start != my_start {
                let mut axes = Vec::new();
                for (idx, (a, b)) in overlap.iter().enumerate() {
                    if idx < i {
                        axes.push((*a, *b));
                    }
                }
                axes.push((my_start, overlap_start - 1));
                for (idx, (a, b)) in self.axes.iter().enumerate() {
                    if idx > i {
                        axes.push((*a, *b));
                    }
                }
                new_cubes.push(Cube { axes })
            }
            if overlap_end != my_end {
                let mut axes = Vec::new();
                for (idx, (a, b)) in overlap.iter().enumerate() {
                    if idx < i {
                        axes.push((*a, *b));
                    }
                }
                axes.push((overlap_end + 1, my_end));
                for (idx, (a, b)) in self.axes.iter().enumerate() {
                    if idx > i {
                        axes.push((*a, *b));
                    }
                }
                new_cubes.push(Cube { axes })
            }
        }
        return new_cubes;
    }
}

impl Command {
    fn new(line: &str) -> Command {
        let (state, ranges) = line.split(" ").collect_tuple().unwrap();
        let axes: Vec<(i128, i128)> = ranges.split(",")
            .map(|x| x.split("=").collect_tuple().unwrap())
            .map(|(axis, values)| values.split("..").map(|x| x.parse::<i128>().unwrap()).collect_tuple().unwrap())
            .collect_vec();
        Command {
            is_on: state == "on",
            cube: Cube { axes },
        }
    }
}

fn process_command(command: &Command, current_on_cubes: &Vec<Cube>) -> Vec<Cube> {
    let mut new_cubes: Vec<Cube> = Vec::new();
    let mut split_cubes = Vec::new();
    if command.is_on {
        new_cubes.push(command.cube.clone());
    }
    for (i, existing_cube) in current_on_cubes.iter().enumerate() {
        if let Some(overlap) = command.cube.find_overlap(existing_cube) {
            split_cubes.push(i);
            let new_cubes_from_split = existing_cube.split_on_overlap(overlap);
            new_cubes.extend(new_cubes_from_split);
        }
    }
    let mut new_current_cubes = current_on_cubes.iter()
        .enumerate()
        .filter(|(idx, c)| !split_cubes.contains(idx))
        .map(|(idx, c)| c.clone())
        .collect_vec();
    new_current_cubes.extend(new_cubes);
    new_current_cubes
}

fn find_non_overlapping_on_cubes(commands: &Vec<Command>) -> Vec<Cube> {
    let mut current_on_cubes = vec![];
    for command in commands {
        current_on_cubes = process_command(command, &current_on_cubes);
    }
    current_on_cubes
}

fn part1(commands: &Vec<Command>) -> i128 {
    let commands_in_range = commands.iter()
        .filter(|&c| c.cube.axes.iter().all(|(x, y)| (x.abs() <= 50) & (y.abs() <= 50)))
        .map(|c| c.clone())
        .collect_vec();
    find_non_overlapping_on_cubes(&commands_in_range)
        .iter()
        .map(|x| x.size())
        .sum()
}

fn part2(commands: &Vec<Command>) -> i128 {
    find_non_overlapping_on_cubes(commands)
        .iter()
        .map(|x| x.size())
        .sum()
}

pub(crate) fn solve() {
    let commands = read_to_string("22.txt").unwrap()
        .lines()
        .map(|line| Command::new(line))
        .collect_vec();
    println!("{}", part1(&commands));
    println!("{}", part2(&commands));
}