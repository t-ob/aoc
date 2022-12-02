use aoc2022::d2;

fn main() {
    let puzzle_inputs = common::io::read_stdin_lines_to_vec::<d2::PuzzleInput>();

    let part_1 = puzzle_inputs
        .iter()
        .map(|puzzle_input| puzzle_input.0.score())
        .sum::<u32>();
    let part_2 = puzzle_inputs
        .iter()
        .map(|puzzle_input| puzzle_input.1.to_round().score())
        .sum::<u32>();

    println!("{}", part_1);
    println!("{}", part_2);
}
