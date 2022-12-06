fn main() {
    let input: String = common::io::read_stdin();

    let input_chars: Vec<char> = input.chars().collect();

    let mut part_1_buf: [usize; 1 << 7] = [0; 1 << 7];
    let mut part_2_buf: [usize; 1 << 7] = [0; 1 << 7];

    let mut i = 0;
    let mut part_1 = None;
    let mut part_2 = None;
    let mut part_1_count = 0;
    let mut part_2_count = 0;
    while i < input_chars.len() {
        if part_1.is_none() && part_1_count == 4 {
            part_1 = Some(i);
        }
        if part_2.is_none() && part_2_count == 14 {
            part_2 = Some(i);
        }

        if part_1.is_some() && part_2.is_some() {
            break;
        }

        let next_char = input_chars[i];

        if part_1.is_none() {
            part_1_buf[next_char as usize] += 1;
            if part_1_buf[next_char as usize] == 1 {
                part_1_count += 1;
            }
            if i >= 4 {
                let prev_char = input_chars[i - 4];
                part_1_buf[prev_char as usize] -= 1;
                if part_1_buf[prev_char as usize] == 0 {
                    part_1_count -= 1;
                }
            }
        }

        if part_2.is_none() {
            part_2_buf[next_char as usize] += 1;
            if part_2_buf[next_char as usize] == 1 {
                part_2_count += 1;
            }
            if i >= 14 {
                let prev_char = input_chars[i - 14];
                part_2_buf[prev_char as usize] -= 1;
                if part_2_buf[prev_char as usize] == 0 {
                    part_2_count -= 1;
                }
            }
        }

        i += 1;
    }

    println!("{}", part_1.unwrap());
    println!("{}", part_2.unwrap());
}
