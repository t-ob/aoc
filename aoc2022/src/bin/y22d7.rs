use aoc2022::d7;

fn main() {
    let terminal_ouput_lines = common::io::read_stdin_lines_to_vec::<d7::TerminalOutputLine>();

    let inode_table = d7::InodeTable::from_terminal_output_lines(&terminal_ouput_lines);

    let dir_inode_sizes = inode_table.dir_sizes(0);
    let part_1: usize = dir_inode_sizes
        .iter()
        .map(|(_, size)| *size)
        .filter(|size| *size < 100000)
        .sum();
    println!("{}", part_1);

    let unused_space = 70000000 - inode_table.dir_size(0);
    let target_size = 30000000 - unused_space;
    let part_2: usize = dir_inode_sizes
        .iter()
        .map(|(_, size)| *size)
        .filter(|size| *size > target_size)
        .min()
        .unwrap();
    println!("{}", part_2);
}
