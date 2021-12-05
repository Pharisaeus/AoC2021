use std::fs;
use std::ops::Not;
use itertools::Itertools;

#[derive(Clone)]
struct Numbers {
    numbers: Vec<BinNumber>,
}

impl Numbers {
    fn bit_size(&self) -> usize {
        self.get_one().bit_size()
    }
    fn bits_at(&self, pos: usize) -> BinNumber {
        BinNumber {
            value: self.numbers
                .iter()
                .map(|number| number.bit_at_position(pos))
                .collect_vec()
        }
    }
    fn filter_by_bit_pos(&self, pos: usize, bit: bool) -> Numbers {
        if self.is_singular() {
            self.clone()
        } else {
            Numbers {
                numbers: self.numbers
                    .iter()
                    .filter(|&number| number.has_bit_at_position(pos, bit))
                    .map(BinNumber::clone)
                    .collect_vec()
            }
        }
    }
    fn is_singular(&self) -> bool {
        self.numbers.len() == 1
    }
    fn get_one(&self) -> BinNumber {
        self.numbers[0].clone()
    }
}

#[derive(Clone)]
struct BinNumber {
    value: Vec<bool>,
}

impl BinNumber {
    fn new(line: &str) -> BinNumber {
        BinNumber {
            value: line.chars()
                .map(|c| c.to_string().as_str().parse::<i32>().unwrap() == 1)
                .collect_vec()
        }
    }
    fn bit_at_position(&self, pos: usize) -> bool {
        self.value[pos]
    }

    fn has_bit_at_position(&self, pos: usize, bit: bool) -> bool {
        self.bit_at_position(pos) == bit
    }

    fn bit_size(&self) -> usize {
        self.value.len()
    }

    fn count_bits(&self, bit: bool) -> usize {
        self.value
            .iter()
            .filter(|&&x| x == bit)
            .count()
    }

    fn most_common(&self) -> bool {
        self.count_bits(true) >= self.count_bits(false)
    }

    fn least_common(&self) -> bool {
        self.most_common().not()
    }

    fn negate(&self) -> BinNumber {
        BinNumber {
            value: self.value.iter()
                .map(|x| x.not())
                .collect_vec()
        }
    }

    fn to_int(&self) -> i32 {
        let bin_string = self.value
            .iter()
            .fold("".to_string(), |acc, &x| acc + (x as i32).to_string().as_str());
        i32::from_str_radix(bin_string.as_str(), 2).unwrap()
    }
}

fn gases(numbers: &Numbers, common: fn(&BinNumber) -> bool) -> i32 {
    (0..numbers.bit_size())
        .fold(numbers.clone(), |acc, pos| acc.filter_by_bit_pos(pos, common(&acc.bits_at(pos))))
        .get_one()
        .to_int()
}

fn part1(numbers: &Numbers) -> i32 {
    let gamma_bits = (0..numbers.bit_size())
        .map(|bit| numbers.bits_at(bit))
        .map(|num| num.most_common())
        .collect_vec();
    let gamma = BinNumber { value: gamma_bits };
    let eps = gamma.negate();
    gamma.to_int() * eps.to_int()
}

fn part2(numbers: &Numbers) -> i32 {
    let oxygen = gases(numbers, BinNumber::most_common);
    let co2 = gases(numbers, BinNumber::least_common);
    oxygen * co2
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("3.txt").unwrap();
    let numbers: Numbers = Numbers {
        numbers: contents.lines()
            .map(|line| BinNumber::new(line))
            .collect()
    };
    println!("{}", part1(&numbers));
    println!("{}", part2(&numbers));
}
