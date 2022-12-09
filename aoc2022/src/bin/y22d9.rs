use std::{collections::HashSet, str::FromStr};

use common::complex::Complex;

#[derive(Debug)]
enum PuzzleInput {
    Right(u8),
    Up(u8),
    Left(u8),
    Down(u8),
}
impl PuzzleInput {
    fn as_complex(&self) -> Complex<i32> {
        match self {
            PuzzleInput::Right(re) => Complex::new(*re as i32, 0),
            PuzzleInput::Up(im) => Complex::new(0, *im as i32),
            PuzzleInput::Left(re) => Complex::new(-(*re as i32), 0),
            PuzzleInput::Down(im) => Complex::new(0, -(*im as i32)),
        }
    }
}

impl FromStr for PuzzleInput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match [&s[0..1], &s[2..]] {
            ["R", d] => Ok(Self::Right(d.parse().unwrap())),
            ["U", d] => Ok(Self::Up(d.parse().unwrap())),
            ["L", d] => Ok(Self::Left(d.parse().unwrap())),
            ["D", d] => Ok(Self::Down(d.parse().unwrap())),
            _ => unreachable!(),
        }
    }
}

fn normalise_move(z: Complex<i32>) -> Complex<i32> {
    match (z.re(), z.im()) {
        (0, 0) => Complex::new(0, 0),
        (0, im) => Complex::new(0, im / im.abs()),
        (re, 0) => Complex::new(re / re.abs(), 0),
        (re, im) => Complex::new(re / re.abs(), im / im.abs()),
    }
}

fn main() {
    let moves = common::io::read_stdin_lines_to_iter::<PuzzleInput>()
        .map(|puzzle_input| puzzle_input.as_complex())
        .collect::<Vec<_>>();

    let mut head = Complex::new(0, 0);
    let mut tail = Complex::new(0, 0);

    let mut seen_end_positions_part_1 = HashSet::new();
    seen_end_positions_part_1.insert((*tail.re(), *tail.im()));

    for z in &moves {
        let total_moves = (z.re() + z.im()).abs();
        let z_normalised = normalise_move(*z);
        let mut current_move = 0;
        while current_move < total_moves {
            current_move += 1;
            head += z_normalised;
            if (head - tail).norm() <= 2 {
                continue;
            }
            tail += normalise_move(head - tail);
            seen_end_positions_part_1.insert((*tail.re(), *tail.im()));
        }
    }

    let part_1 = seen_end_positions_part_1.len();
    println!("{}", part_1);

    let mut knots = vec![Complex::new(0, 0); 10];
    let mut seen_end_positions_part_2 = HashSet::new();
    seen_end_positions_part_2.insert((*knots[9].re(), *knots[9].im()));

    for z in &moves {
        let total_moves = (z.re() + z.im()).abs();
        let z_normalised = normalise_move(*z);
        let mut current_move = 0;
        while current_move < total_moves {
            current_move += 1;
            knots[0] += z_normalised;
            for k in 0..9 {
                let head_knot = knots[k];
                let next_knot = knots[k + 1];
                if (head_knot - next_knot).norm() <= 2 {
                    continue;
                }
                let next_move = normalise_move(head_knot - next_knot);
                knots[k + 1] += next_move;
                if k == 8 {
                    seen_end_positions_part_2.insert((*knots[k + 1].re(), *knots[k + 1].im()));
                }
            }
        }
    }

    let part_2 = seen_end_positions_part_2.len();
    println!("{}", part_2);
}
