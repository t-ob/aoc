use std::io::stdin;

static SCORES_1: [[u16; 1 << 7]; 1 << 7] = {
    let mut scores = [[0; 1 << 7]; 1 << 7];

    scores['A' as usize]['X' as usize] = 4;
    scores['A' as usize]['Y' as usize] = 8;
    scores['A' as usize]['Z' as usize] = 3;
    scores['B' as usize]['X' as usize] = 1;
    scores['B' as usize]['Y' as usize] = 5;
    scores['B' as usize]['Z' as usize] = 9;
    scores['C' as usize]['X' as usize] = 7;
    scores['C' as usize]['Y' as usize] = 2;
    scores['C' as usize]['Z' as usize] = 6;

    scores
};

static SCORES_2: [[u16; 1 << 7]; 1 << 7] = {
    let mut scores = [[0; 1 << 7]; 1 << 7];

    scores['A' as usize]['X' as usize] = 3;
    scores['A' as usize]['Y' as usize] = 4;
    scores['A' as usize]['Z' as usize] = 8;
    scores['B' as usize]['X' as usize] = 1;
    scores['B' as usize]['Y' as usize] = 5;
    scores['B' as usize]['Z' as usize] = 9;
    scores['C' as usize]['X' as usize] = 2;
    scores['C' as usize]['Y' as usize] = 6;
    scores['C' as usize]['Z' as usize] = 7;

    scores
};

fn main() {
    let mut part_1 = 0;
    let mut part_2 = 0;
    let stdin = stdin();
    for line in stdin.lines().filter_map(Result::ok) {
        let mut chars = line.chars();

        let lhs = chars.next().unwrap();
        let rhs = chars.last().unwrap();

        part_1 += SCORES_1[lhs as usize][rhs as usize];
        part_2 += SCORES_2[lhs as usize][rhs as usize];
    }

    println!("{}", part_1);
    println!("{}", part_2);
}
