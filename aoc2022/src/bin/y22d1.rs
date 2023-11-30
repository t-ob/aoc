use std::collections::BinaryHeap;
use std::io::stdin;

fn main() {
    let mut weights: Vec<i32> = vec![0; 1 << 10];

    let mut buffer = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let mut i = 0;
    let stdin = stdin();
    for line in stdin.lines().filter_map(Result::ok) {
        if line.is_empty() {
            i += 1;
            continue;
        }

        weights[i] += line.parse::<i32>().unwrap();
    }

    let mut top_slice_idx = 0;
    for weight in weights[..=i].iter() {
        let x = buffer[top_slice_idx];
        let y = buffer[top_slice_idx + 1];
        let z = buffer[top_slice_idx + 2];

        buffer[0] = x;
        buffer[1] = y;
        buffer[2] = z;

        buffer[3] = *weight;
        buffer[4] = y;
        buffer[5] = z;

        buffer[6] = y;
        buffer[7] = *weight;
        buffer[8] = z;

        buffer[9] = y;
        buffer[10] = z;
        buffer[11] = *weight;

        let top_slice = ((weight - x).signum() + 1) / 2 + ((weight - y).signum() + 1) / 2 + ((weight - z).signum() + 1) / 2;

        top_slice_idx = 3 * top_slice as usize;
    }

    let part_1 = buffer[top_slice_idx + 2];
    let part_2 = buffer[top_slice_idx] + buffer[top_slice_idx + 1] + buffer[top_slice_idx + 2];

    println!("{}", part_1);
    println!("{}", part_2);
}
