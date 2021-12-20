use std::collections::HashSet;
use std::fs::read_to_string;
use itertools::Itertools;

struct Image {
    data: HashSet<(i32, i32)>,
    enh: Vec<char>,
    flipper: bool,
}

impl Image {
    fn enhance(&self) -> Image {
        let (sx, ex, sy, ey) = self.ranges();
        let mut res = HashSet::new();
        for i in sx - 3..ex + 4 {
            for j in sy - 3..ey + 4 {
                if self.is_enhanced_white(i, j, sx, ex, sy, ey) {
                    res.insert((i, j));
                }
            }
        }
        Image {
            data: res,
            enh: self.enh.clone(),
            flipper: !self.flipper,
        }
    }

    fn is_enhanced_white(&self, i: i32, j: i32, sx: i32, ex: i32, sy: i32, ey: i32) -> bool {
        let indices = [(i - 1, j - 1), (i - 1, j), (i - 1, j + 1), (i, j - 1), (i, j), (i, j + 1), (i + 1, j - 1), (i + 1, j), (i + 1, j + 1)];
        let mut binary = vec![];
        for (a, b) in indices {
            if self.data.contains(&(a, b)) {
                binary.push(1);
            } else if (a < sx) | (a > ex) | (b < sy) | (b > ey) {
                if self.flipper & (self.enh[0] == '#') {
                    binary.push(1);
                } else {
                    binary.push(0);
                }
            } else {
                binary.push(0);
            }
        }
        let bin_string = binary
            .iter()
            .fold("".to_string(), |acc, &x| acc + x.to_string().as_str());
        let index = usize::from_str_radix(bin_string.as_str(), 2).unwrap();
        self.enh[index] == '#'
    }

    fn ranges(&self) -> (i32, i32, i32, i32) {
        let &sx = self.data.iter()
            .map(|(x, y)| x)
            .min().unwrap();
        let &ex = self.data.iter()
            .map(|(x, y)| x)
            .max().unwrap();
        let &sy = self.data.iter()
            .map(|(x, y)| y)
            .min().unwrap();
        let &ey = self.data.iter()
            .map(|(x, y)| y)
            .max().unwrap();
        (sx, ex, sy, ey)
    }

    fn white(&self) -> usize {
        self.data.len()
    }
}

fn enhance(img: &Image, n: i32) -> usize {
    let mut image = img.enhance();
    for _ in 0..n - 1 {
        image = image.enhance();
    }
    image.white()
}

fn part1(img: &Image) -> usize {
    enhance(img, 2)
}

fn part2(img: &Image) -> usize {
    enhance(img, 50)
}

pub(crate) fn solve() {
    let dataset = read_to_string("20.txt").unwrap();
    let x = dataset.split("\n\n").collect_vec();
    let enh = x[0].chars().collect_vec();
    let mut collapsed = HashSet::new();
    for (i, line) in x[1].split("\n").enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                collapsed.insert((i as i32, j as i32));
            }
        }
    }
    let img = &Image {
        data: collapsed,
        enh,
        flipper: false,
    };
    println!("{}", part1(img));
    println!("{}", part2(img));
}
