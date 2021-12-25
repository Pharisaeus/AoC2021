use std::collections::HashSet;
use std::fs::read_to_string;
use itertools::Itertools;


fn wrap(index: usize, max: usize) -> usize {
    match index {
        0 => max - 1,
        _ => index - 1
    }
}

struct Board {
    board: Vec<Vec<String>>,
    free: HashSet<(usize, usize)>,
    targets_east: HashSet<(usize, usize)>,
    targets_south: HashSet<(usize, usize)>,
}

impl Board {
    fn new(board: Vec<Vec<String>>) -> Board {
        let mut b = Board {
            board,
            free: Default::default(),
            targets_east: Default::default(),
            targets_south: Default::default(),
        };
        b.compute_targets();
        b
    }

    fn step(&mut self) {
        let w = self.width();
        let h = self.height();
        let changes_east = self.free.intersection(&self.targets_east).map(|x| *x).collect_vec();
        for (i, j) in changes_east {
            self.board[i][j] = ">".to_string();
            self.board[i][wrap(j, w)] = ".".to_string();
            self.free.remove(&(i, j));
            self.free.insert((i, wrap(j, w)));
        }

        let changes_south = self.free.intersection(&self.targets_south).map(|x| *x).collect_vec();
        for (i, j) in changes_south {
            self.board[i][j] = "v".to_string();
            self.board[wrap(i, h)][j] = ".".to_string();
            self.free.remove(&(i, j));
            self.free.insert((wrap(i, h), j));
        }
        self.compute_targets()
    }

    fn compute_targets(&mut self) {
        self.free = ((0..self.height())
            .flat_map(|i| (0..self.width())
                .map(move |j| (i, j))))
            .filter(|(i, j)| self.board[*i][*j] == ".")
            .collect();

        self.targets_east = ((0..self.height())
            .flat_map(|i| (0..self.width())
                .map(move |j| (i, j))))
            .filter(|(i, j)| self.board[*i][*j] == ">")
            .map(|(i, j)| (i, (j + 1) % self.width()))
            .collect();

        self.targets_south = ((0..self.height())
            .flat_map(|i| (0..self.width())
                .map(move |j| (i, j))))
            .filter(|(i, j)| self.board[*i][*j] == "v")
            .map(|(i, j)| ((i + 1) % self.height(), j))
            .collect();
    }

    fn blocked(&self) -> bool {
        self.free.intersection(&self.targets_east).count() == 0
    }

    fn height(&self) -> usize {
        self.board.len()
    }

    fn width(&self) -> usize {
        self.board[0].len()
    }
}

pub(crate) fn solve() {
    let board = read_to_string("25.txt").unwrap()
        .lines()
        .map(|line| line.chars().map(|x| x.to_string()).collect_vec())
        .collect_vec();
    let mut b = Board::new(board);
    let mut steps = 1;
    while !b.blocked() {
        b.step();
        steps += 1;
    }
    println!("{}", steps)
}