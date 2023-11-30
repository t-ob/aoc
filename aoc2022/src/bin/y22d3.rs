use std::str::FromStr;

use aoc2022::d3;

const PRIORITIES: [u32; 1 << 7] = {
    let mut priorities = [0; 1 << 7];

    priorities['a' as usize] = 1;

    priorities
};

#[derive(Debug)]
struct Rucksack(Vec<char>, Vec<char>);

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_at(s.len() / 2);

        Ok(Self(left.chars().collect(), right.chars().collect()))
    }
}

fn sum_priorities(set: u64) -> u64 {
    let mut set_mut = set;
    let mut result = 0;
    let mut priority = 0;
    while set_mut > 0 {
        result += priority * (set_mut & 1);
        set_mut >>= 1;
        priority += 1;
    }

    result
}

fn main() {
    let rucksacks = common::io::read_stdin_lines_to_vec::<Rucksack>();

    let mut part_1 = 0;
    for rucksack in &rucksacks {
        let mut set = 0u64;
        let mut other_set = 0u64;

        for char in &rucksack.0 {
            set |= d3::ITEMS[*char as usize];
        }

        for char in &rucksack.1 {
            other_set |= d3::ITEMS[*char as usize];
        }

        set &= other_set;

        part_1 += sum_priorities(set);
    }

    let mut part_2 = 0;
    for group in rucksacks.chunks_exact(3) {
        let mut set = 0u64;
        let mut other_set_1 = 0u64;
        let mut other_set_2 = 0u64;

        for (rucksack, s) in group
            .iter()
            .zip([&mut set, &mut other_set_1, &mut other_set_2])
        {
            for char in &rucksack.0 {
                *s |= d3::ITEMS[*char as usize];
            }

            for char in &rucksack.1 {
                *s |= d3::ITEMS[*char as usize];
            }
        }

        set &= other_set_1;
        set &= other_set_2;

        part_2 += sum_priorities(set);
    }

    println!("{}", part_1);
    println!("{}", part_2);
}
