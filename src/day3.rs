use std::fs;
use std::ops::Not;
use std::ptr::null;
use itertools::{Itertools, put_back};

struct Numbers {
    numbers: Vec<BinNumber>,
}

impl Numbers {
    fn bit_size(&self) -> usize {
        return self.get_one().bit_size();
    }
    fn bits_at(&self, pos: usize) -> BinNumber {
        return BinNumber {
            value: self.numbers
                .iter()
                .map(|number| number.bit_at_position(pos))
                .collect_vec()
        };
    }
    fn filter_by_bit_pos(&self, pos: usize, bit: i32) -> Numbers {
        return if self.is_singular() {
            self.clone()
        } else {
            Numbers {
                numbers: self.numbers
                    .iter()
                    .filter(|&number| number.has_bit_at_position(pos, bit))
                    .map(BinNumber::clone)
                    .collect_vec()
            }
        };
    }
    fn is_singular(&self) -> bool {
        return self.numbers.len() == 1;
    }
    fn get_one(&self) -> BinNumber {
        return self.numbers[0].clone();
    }
    fn clone(&self) -> Numbers {
        return Numbers {
            numbers: self.numbers
                .iter()
                .map(BinNumber::clone)
                .collect_vec()
        };
    }
}

struct BinNumber {
    value: Vec<i32>,
}

impl BinNumber {
    fn new(line: &str) -> BinNumber {
        return BinNumber {
            value: line.chars()
                .map(|c| c.to_string().as_str().parse().unwrap())
                .collect_vec()
        };
    }
    fn bit_at_position(&self, pos: usize) -> i32 {
        return self.value[pos];
    }

    fn has_bit_at_position(&self, pos: usize, bit: i32) -> bool {
        return self.bit_at_position(pos) == bit;
    }

    fn bit_size(&self) -> usize {
        return self.value.len();
    }

    fn count_bits(&self, bit: i32) -> usize {
        return self.value
            .iter()
            .filter(|&&x| x == bit)
            .count();
    }

    fn most_common(&self) -> i32 {
        return (self.count_bits(1) >= self.count_bits(0)) as i32;
    }

    fn least_common(&self) -> i32 {
        return (self.most_common() != 1) as i32;
    }

    fn negate(&self) -> BinNumber {
        return BinNumber {
            value: self.value.iter()
                .map(|x| (x + 1) % 2)
                .collect_vec()
        };
    }

    fn to_int(&self) -> i32 {
        let bin_string = self.value
            .iter()
            .fold("".to_string(), |acc, &x| acc + x.to_string().as_str());
        return i32::from_str_radix(bin_string.as_str(), 2).unwrap();
    }

    fn clone(&self) -> BinNumber {
        return BinNumber { value: self.value.clone() };
    }
}

fn gases(numbers: &Numbers, common: fn(&BinNumber) -> i32) -> i32 {
    return (0..numbers.bit_size())
        .fold(numbers.clone(), |acc, pos| acc.filter_by_bit_pos(pos, common(&acc.bits_at(pos))))
        .get_one()
        .to_int();
}

fn part1(numbers: &Numbers) -> i32 {
    let gamma_bits = (0..numbers.bit_size())
        .map(|bit| numbers.bits_at(bit))
        .map(|num| num.most_common())
        .collect_vec();
    let gamma = BinNumber { value: gamma_bits };
    let eps = gamma.negate();
    return gamma.to_int() * eps.to_int();
}

fn part2(numbers: &Numbers) -> i32 {
    let oxygen = gases(numbers, BinNumber::most_common);
    let co2 = gases(numbers, BinNumber::least_common);
    return oxygen * co2;
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
