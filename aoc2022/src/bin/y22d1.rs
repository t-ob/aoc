use std::collections::BinaryHeap;

fn main() {
    let inventories = common::io::collect_stdin_lines::<u32>("\n\n");

    let mut heap =
        BinaryHeap::<u32>::from_iter(inventories.iter().map(|inventory| inventory.iter().sum()));

    let part_1 = *heap.peek().unwrap();
    let part_2 = heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap();

    println!("{}", part_1);
    println!("{}", part_2);
}
