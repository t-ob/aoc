use std::str::FromStr;

const A: usize = 'A' as usize - 'A' as usize;
const B: usize = 'B' as usize - 'A' as usize;
const C: usize = 'C' as usize - 'A' as usize;
const X: usize = 'X' as usize - 'A' as usize;
const Y: usize = 'Y' as usize - 'A' as usize;
const Z: usize = 'Z' as usize - 'A' as usize;

const SCORES: [u16; 1 << 10] = {
    let mut scores = [0; 1 << 10];

    scores[A + 26 * X] = 4;
    scores[A + 26 * Y] = 8;
    scores[A + 26 * Z] = 3;
    scores[B + 26 * X] = 1;
    scores[B + 26 * Y] = 5;
    scores[B + 26 * Z] = 9;
    scores[C + 26 * X] = 7;
    scores[C + 26 * Y] = 2;
    scores[C + 26 * Z] = 6;

    scores
};

const OUTCOMES: [u16; 1 << 10] = {
    let mut outcomes = [0; 1 << 10];

    outcomes[A + 26 * X] = SCORES[A + 26 * Z];
    outcomes[A + 26 * Y] = SCORES[A + 26 * X];
    outcomes[A + 26 * Z] = SCORES[A + 26 * Y];
    outcomes[B + 26 * X] = SCORES[B + 26 * X];
    outcomes[B + 26 * Y] = SCORES[B + 26 * Y];
    outcomes[B + 26 * Z] = SCORES[B + 26 * Z];
    outcomes[C + 26 * X] = SCORES[C + 26 * Y];
    outcomes[C + 26 * Y] = SCORES[C + 26 * Z];
    outcomes[C + 26 * Z] = SCORES[C + 26 * X];

    outcomes
};

#[derive(Debug)]
struct Idx(usize);

impl FromStr for Idx {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        Ok(Self(
            (chars.nth(0).unwrap() as usize - 'A' as usize)
                + 26 * (chars.nth(1).unwrap() as usize - 'A' as usize),
        ))
    }
}

fn main() {
    let puzzle_inputs = common::io::read_stdin_lines_to_vec::<Idx>();

    let part_1 = puzzle_inputs
        .iter()
        .map(|puzzle_input| SCORES[puzzle_input.0])
        .sum::<u16>();
    let part_2 = puzzle_inputs
        .iter()
        .map(|puzzle_input| OUTCOMES[puzzle_input.0])
        .sum::<u16>();

    println!("{}", part_1);
    println!("{}", part_2);
}
