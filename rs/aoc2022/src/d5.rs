use std::{collections::VecDeque, str::FromStr};

#[derive(Clone)]
pub struct Move(usize, usize, usize);

#[derive(Clone)]
pub struct PuzzleInput {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

impl FromStr for PuzzleInput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks = vec![];
        let mut moves = vec![];

        let mut processing_stack = true;
        for line in s.lines() {
            if line.starts_with(" 1") {
                continue;
            }
            if line.is_empty() {
                processing_stack = false;
                continue;
            }
            if processing_stack {
                for (idx, c) in line.chars().enumerate() {
                    if !c.is_ascii_uppercase() {
                        continue;
                    }

                    let stack_idx = idx / 4;
                    while stacks.len() <= stack_idx {
                        stacks.push(VecDeque::new());
                    }

                    stacks[stack_idx].push_front(c);
                }
            } else {
                let mut tokens = line.split_ascii_whitespace();

                moves.push(Move(
                    tokens
                        .nth(1)
                        .unwrap()
                        .parse()
                        .expect("Unable to parse amount"),
                    tokens
                        .nth(1)
                        .unwrap()
                        .parse::<usize>()
                        .expect("Unable to parse from")
                        - 1,
                    tokens
                        .nth(1)
                        .unwrap()
                        .parse::<usize>()
                        .expect("Unable to parse to")
                        - 1,
                ));
            }
        }

        Ok(Self { stacks, moves })
    }
}

pub enum Mode {
    CrateMover9000,
    CrateMover9001,
}

impl PuzzleInput {
    pub fn execute(&mut self, mode: Mode) {
        let mut queue = VecDeque::new();

        for mv in self.moves.drain(..) {
            for _ in 0..mv.0 {
                let next_char = self.stacks[mv.1].pop_back().unwrap();
                match mode {
                    Mode::CrateMover9000 => queue.push_back(next_char),
                    Mode::CrateMover9001 => queue.push_front(next_char),
                }
            }
            for c in queue.drain(..) {
                self.stacks[mv.2].push_back(c);
            }
        }
    }

    pub fn top_crates(&self) -> Vec<char> {
        self.stacks
            .iter()
            .map(|stack| *stack.back().unwrap())
            .collect()
    }
}
