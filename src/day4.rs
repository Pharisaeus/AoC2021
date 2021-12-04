use std::fs;
use itertools::Itertools;

struct Field {
    value: i32,
    marked: bool,
}

impl Field {
    fn is_marked(&self) -> bool {
        return self.marked;
    }
    fn mark(&self, value: i32) -> Field {
        return Field { value: self.value, marked: self.marked | (self.value == value) };
    }
}

struct Board {
    numbers: Vec<Vec<Field>>,
}

impl Board {
    fn new(block: &str) -> Board {
        let numbers = block.split("\n")
            .map(|line| line.split_whitespace()
                .map(|element| Field { value: element.parse().unwrap(), marked: false })
                .collect_vec())
            .collect_vec();
        return Board { numbers };
    }
    fn mark(&self, value: i32) -> Board {
        return Board {
            numbers: self.numbers.iter()
                .map(|row| row.iter()
                    .map(|field| field.mark(value))
                    .collect_vec())
                .collect_vec()
        };
    }
    fn is_winning(&self) -> bool {
        return self.is_row_marked() | self.is_column_marked();
    }
    fn cols(&self) -> usize {
        return self.numbers[0].len();
    }
    fn rows(&self) -> usize {
        return self.numbers.len();
    }
    fn is_row_marked(&self) -> bool {
        return (0..self.rows())
            .any(|row_idx| (0..self.cols())
                .all(|col_idx| self.numbers[row_idx][col_idx].is_marked()));
    }
    fn is_column_marked(&self) -> bool {
        return (0..self.cols())
            .any(|col_idx| (0..self.rows())
                .all(|row_idx| self.numbers[row_idx][col_idx].is_marked()));
    }
    fn get_unmarked(&self) -> Vec<i32> {
        return self.numbers.iter()
            .flat_map(|row| row.iter()
                .filter(|element| element.is_marked() == false)
                .map(|x| x.value))
            .collect_vec();
    }
    fn clone(&self) -> Board {
        return Board {
            numbers: self.numbers
                .iter()
                .map(|row| row.iter()
                    .map(|f| Field { value: f.value, marked: f.marked })
                    .collect_vec())
                .collect_vec()
        };
    }
}

struct Boards {
    boards: Vec<Board>,
}

impl Boards {
    fn mark(&self, value: i32) -> Boards {
        return Boards {
            boards: self.boards.iter()
                .map(|board| board.mark(value))
                .collect_vec()
        };
    }
    fn is_any_winning(&self) -> bool {
        return self.boards
            .iter()
            .any(|board| board.is_winning());
    }
    fn is_all_winning(&self) -> bool {
        return self.boards
            .iter()
            .all(|board| board.is_winning());
    }
    fn get_winning_score(&self) -> i32 {
        return self.boards.iter()
            .find_or_first(|&board| board.is_winning())
            .map(|board| board.get_unmarked().iter().sum())
            .unwrap();
    }
    fn filter_out_winning(&self) -> Boards {
        return Boards {
            boards: self.boards.iter()
                .filter(|board| board.is_winning() == false)
                .map(|board| board.clone())
                .collect_vec()
        };
    }
}

struct InputData {
    numbers: Vec<i32>,
    boards: Boards,
}

fn part1(data: &InputData) -> i32 {
    let mut boards = data.boards.filter_out_winning();
    for number in data.numbers.iter() {
        boards = boards.mark(*number);
        if boards.is_any_winning() {
            return boards.get_winning_score() * number;
        }
    }
    return -1;
}

fn part2(data: &InputData) -> i32 {
    let mut boards = data.boards.filter_out_winning();
    for number in data.numbers.iter() {
        boards = boards.mark(*number);
        if boards.is_all_winning() {
            return boards.get_winning_score() * number;
        } else {
            boards = boards.filter_out_winning();
        }
    }
    return -1;
}

fn load_data(name: &str) -> InputData {
    let contents = fs::read_to_string(name).unwrap();
    let blocks = contents.split("\r\n\r\n").collect_vec();
    let numbers: Vec<i32> = blocks[0].split(",")
        .map(|x| x.parse().unwrap())
        .collect_vec();
    let boards = blocks[1..]
        .iter()
        .map(|&block| Board::new(block))
        .collect_vec();
    return InputData { numbers, boards: Boards { boards } };
}

pub(crate) fn solve() {
    let data = load_data("4.txt");
    println!("{}", part1(&data));
    println!("{}", part2(&data));
}
