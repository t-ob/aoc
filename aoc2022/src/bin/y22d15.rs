use std::collections::{BinaryHeap, HashSet};

use common::complex::Complex;

fn main() {
    let data = common::io::map_stdin_lines_to_vec(|line| {
        let mut nums = [0; 4];
        let groups = line.split('=');

        for (idx, group) in groups.skip(1).enumerate() {
            let group_chars = group.chars().collect::<Vec<_>>();
            let mut i = 0;
            while i < group_chars.len() && !(group_chars[i] == ':' || group_chars[i] == ',') {
                i += 1;
            }
            nums[idx] = group[..i].parse().unwrap();
        }

        let sensor = Complex::new(nums[0], nums[1]);
        let beacon = Complex::new(nums[2], nums[3]);

        let delta = beacon - sensor;
        let delta_re: i32 = delta.re();
        let delta_im: i32 = delta.im();
        let l1_distance = delta_re.abs() + delta_im.abs();

        (sensor, beacon, l1_distance)
    });

    // Part 1
    let target = 2000000;

    let target_imag_axis = Complex::new(0, target);
    let mut target_beacons = HashSet::new();
    let mut target_beacon_real_values = BinaryHeap::new();
    let mut intervals = BinaryHeap::new();

    for (sensor, beacon, distance) in &data {
        if beacon.im() == target {
            if !target_beacons.contains(&beacon) {
                target_beacon_real_values.push(beacon.re())
            }
            target_beacons.insert(beacon);
        }
        let delta_imag_axis = target_imag_axis - Complex::new(0, sensor.im());

        if delta_imag_axis.im().abs() <= *distance {
            let delta_real = distance - delta_imag_axis.im().abs();
            let interval = (sensor.re() - delta_real, sensor.re() + delta_real);
            intervals.push((interval.1, interval));
        }
    }

    let mut part_1 = 0;

    while !intervals.is_empty() {
        let mut in_interval = 0;
        let (_, (l, r)) = intervals.pop().unwrap();
        let mut curr_l = l;
        while let Some((_, (next_l, next_r))) = intervals.peek() {
            if *next_r >= curr_l {
                curr_l = curr_l.min(*next_l);
                intervals.pop();
            } else {
                break
            }
        }

        while let Some(re) = target_beacon_real_values.peek() {
            if *re < curr_l {
                break;
            }

            if *re <= r {
                in_interval += 1;
            }

            target_beacon_real_values.pop();
        }

        part_1 += r - curr_l + 1 - in_interval;
    }

    println!("{}", part_1);

    // Part 2
    let mut found = false;
    let mut part_2 = 0;

    for target in 0..=4000000 {
        if found { break; }
        let target_imag_axis = Complex::new(0, target);
        let mut intervals = BinaryHeap::new();

        for (sensor, _, distance) in &data {
            let delta_imag_axis = target_imag_axis - Complex::new(0, sensor.im());

            if delta_imag_axis.im().abs() <= *distance {
                let delta_real = distance - delta_imag_axis.im().abs();
                let interval = (sensor.re() - delta_real, sensor.re() + delta_real);
                if interval.1 >= 0 && interval.0 <= 4000000 {
                    intervals.push((interval.1, interval));
                }
            }
        }


        while !intervals.is_empty() {
            if found { break; }
            let (_, (l, _)) = intervals.pop().unwrap();

            let mut curr_l = l;
            while let Some((_, (next_l, next_r))) = intervals.peek() {
                if *next_r >= curr_l {
                    curr_l = curr_l.min(*next_l);
                    intervals.pop();
                } else {
                    part_2 = 4000000 * (*next_r + 1) as i64 + target as i64;
                    found = true;
                    break;
                }
            }
        }
    }

    println!("{}", part_2);
}
