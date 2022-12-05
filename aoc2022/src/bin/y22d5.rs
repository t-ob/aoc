use aoc2022::d5;

fn main() {
    let mut cargo_crane_part_1: d5::PuzzleInput = common::io::read_stdin();
    let mut cargo_crane_part_2 = cargo_crane_part_1.clone();

    cargo_crane_part_1.execute(d5::Mode::CrateMover9000);
    let part_1 = String::from_iter(cargo_crane_part_1.top_crates());

    cargo_crane_part_2.execute(d5::Mode::CrateMover9001);
    let part_2 = String::from_iter(cargo_crane_part_2.top_crates());

    println!("{}", part_1);
    println!("{}", part_2)
}
