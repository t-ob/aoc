use std::{str::FromStr, collections::VecDeque};

use common::io;

#[derive(Debug)]
enum Move {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();
        match (tokens.next(), tokens.next().map(|s| s.parse())) {
            (Some("forward"), Some(Ok(x))) => Ok(Self::Forward(x)),
            (Some("down"), Some(Ok(x))) => Ok(Self::Down(x)),
            (Some("up"), Some(Ok(x))) => Ok(Self::Up(x)),
            _ => Err(String::from("Invalid move")),
        }
    }
}

fn main() {
    let nums = io::map_stdin_lines_to_vec(|line| u16::from_str_radix(&line, 2));

    let ddd = nums.iter().max().unwrap().next_power_of_two();

    let mask = ddd - 1;

    let mut log = 0;
    let mut eee = ddd;
    while eee > 0 {
        log += 1;
        eee >>= 1;
    }
    log -= 1;

    let mut one_counts = vec![];
    let mut zero_counts = vec![];

    for _ in 0..log {
        one_counts.push(0);
        zero_counts.push(0)
    }

    for cc in &nums {
        let ones = cc;
        let zeroes = ones ^ mask;

        for i in 0..log {
            one_counts[i] += (ones >> log - i - 1) & 1;
            zero_counts[i] += (zeroes >> log - i - 1) & 1;
        }
    }

    let gamma = one_counts.iter().zip(zero_counts.iter()).fold(0, |acc, (x, y)| acc << 1 | (x > y) as u16);
    let epsilon = gamma ^ mask;

    let part_1 = gamma as u32 * epsilon as u32;
    
    println!("{}", part_1);

    let mut queue_o2 = VecDeque::from(nums);
    let mut queue_co2 = queue_o2.clone();

    let mut ones_p2 = VecDeque::new();
    let mut zeroes_p2 = VecDeque::new();

    let mut shift = 0;
    while queue_o2.len() > 1 {
        ones_p2.clear();
        zeroes_p2.clear();
        let next_o2s = queue_o2.drain(..);

        for next_o2 in next_o2s {
            match next_o2 >> (log - (1 + shift)) & 1 {
                0 => zeroes_p2.push_back(next_o2),
                1 => ones_p2.push_back(next_o2),
                _ => unreachable!()
            }
            
        }

        if ones_p2.len() >= zeroes_p2.len() {
            queue_o2.append(&mut ones_p2);
        } else {
            queue_o2.append(&mut zeroes_p2)
        }

        shift += 1;
    }

    shift = 0;
    while queue_co2.len() > 1 {
        ones_p2.clear();
        zeroes_p2.clear();
        let next_co2s = queue_co2.drain(..);

        for next_co2 in next_co2s {
            match next_co2 >> (log - (1 + shift)) & 1 {
                0 => zeroes_p2.push_back(next_co2),
                1 => ones_p2.push_back(next_co2),
                _ => unreachable!()
            }
        }

        if ones_p2.len() < zeroes_p2.len() {
            queue_co2.append(&mut ones_p2);
        } else {
            queue_co2.append(&mut zeroes_p2)
        }

        shift += 1;
    }

    let part_2 = queue_co2.pop_front().unwrap() as u64 * queue_o2.pop_front().unwrap() as u64;

    println!("{}", part_2);
}
