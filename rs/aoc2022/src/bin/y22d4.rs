use std::str::FromStr;

#[derive(Debug)]
struct AssignmentPair(u128, u128);

impl AssignmentPair {
    pub fn fully_overlapping(&self) -> bool {
        let overlap = self.0 & self.1;
        overlap == self.0 || overlap == self.1
    }

    pub fn partially_overlapping(&self) -> bool {
        let overlap = self.0 & self.1;
        overlap > 0
    }
}

impl FromStr for AssignmentPair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(',').unwrap();
        let (left_lower, left_upper) = left.split_once('-').unwrap();
        let (right_lower, right_upper) = right.split_once('-').unwrap();

        let mut i: usize = left_lower.parse().unwrap();
        let mut j: usize = left_upper.parse().unwrap();

        let mut first = 0;
        while i <= j {
            first |= 1 << i;
            i += 1;
        }

        i = right_lower.parse().unwrap();
        j = right_upper.parse().unwrap();

        let mut second = 0;
        while i <= j {
            second |= 1 << i;
            i += 1;
        }

        Ok(Self(first, second))
    }
}

fn main() {
    let assignment_pairs = common::io::read_stdin_lines_to_vec::<AssignmentPair>();

    let part_1 = assignment_pairs
        .iter()
        .filter(|pair| pair.fully_overlapping())
        .count();
    let part_2 = assignment_pairs
        .iter()
        .filter(|pair| pair.partially_overlapping())
        .count();

    println!("{}", part_1);
    println!("{}", part_2);
}
