use common::io;

fn main() {
    let depths = io::read_stdin_lines_to_vec::<u16>();

    let part_1 = depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(x, y)| x < y)
        .count();
    let part_2 = depths
        .iter()
        .zip(depths.iter().skip(3))
        .filter(|(x, y)| x < y)
        .count();

    println!("{}", part_1);
    println!("{}", part_2);
}
