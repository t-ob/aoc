use std::str::FromStr;

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
    let moves = io::read_stdin_lines_to_vec::<Move>();

    let mut horizontal = 0;
    let mut depth = 0;
    for mv in &moves {
        match mv {
            Move::Forward(x) => horizontal += x,
            Move::Down(x) => depth += x,
            Move::Up(x) => depth -= x,
        }
    }

    let part_1 = horizontal * depth;

    println!("{}", part_1);

    horizontal = 0;
    depth = 0;
    let mut aim = 0;

    for mv in &moves {
        match mv {
            Move::Forward(x) => {
                horizontal += x;
                depth += aim * x;
            }
            Move::Down(x) => aim += x,
            Move::Up(x) => aim -= x,
        }
    }

    let part_2 = horizontal * depth;
    println!("{}", part_2);
}
