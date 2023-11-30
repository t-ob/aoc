use std::str::FromStr;

#[derive(Debug, Clone)]
enum PacketValue {
    Val(i32),
    Ref(usize),
}

#[derive(Debug, Clone)]
struct Packet {
    values: Vec<Vec<PacketValue>>,
    divider: bool,
}

impl Packet {
    fn is_divider(&self) -> bool {
        self.divider
    }
    fn compare(&self, rhs: &Self) -> bool {
        let mut lhs_ref_idx = vec![0];
        let mut rhs_ref_idx = vec![0];

        let mut lhs_ref_value_idx = vec![0];
        let mut rhs_ref_value_idx = vec![0];

        loop {
            match (
                &self.values[*lhs_ref_idx.last().unwrap()].get(*lhs_ref_value_idx.last().unwrap()),
                &rhs.values[*rhs_ref_idx.last().unwrap()].get(*rhs_ref_value_idx.last().unwrap()),
            ) {
                (Some(PacketValue::Val(l)), Some(PacketValue::Val(r))) if *l == *r => {
                    let curr_lhs_ref_value_idx = lhs_ref_value_idx.last_mut().unwrap();
                    *curr_lhs_ref_value_idx += 1;
                    let curr_rhs_ref_value_idx = rhs_ref_value_idx.last_mut().unwrap();
                    *curr_rhs_ref_value_idx += 1;
                }
                (Some(PacketValue::Val(l)), Some(PacketValue::Val(r))) => return *l < *r,
                (
                    Some(PacketValue::Ref(next_rhs_ref_idx)),
                    Some(PacketValue::Ref(next_lhs_ref_idx)),
                ) => {
                    let curr_lhs_ref_value_idx = lhs_ref_value_idx.last_mut().unwrap();
                    *curr_lhs_ref_value_idx += 1;

                    let curr_rhs_ref_value_idx = rhs_ref_value_idx.last_mut().unwrap();
                    *curr_rhs_ref_value_idx += 1;

                    lhs_ref_idx.push(*next_rhs_ref_idx);
                    rhs_ref_idx.push(*next_lhs_ref_idx);
                    lhs_ref_value_idx.push(0);
                    rhs_ref_value_idx.push(0);
                }
                (Some(PacketValue::Ref(next_rhs_ref_idx)), Some(PacketValue::Val(r))) => {
                    let mut curr_ref_idx = *next_rhs_ref_idx;
                    while let Some(PacketValue::Ref(next_ref_idx)) =
                        self.values[curr_ref_idx].get(0)
                    {
                        curr_ref_idx = *next_ref_idx;
                    }

                    match self.values[curr_ref_idx].get(0) {
                        None => return true,
                        Some(PacketValue::Val(l)) if l == r => {
                            if self.values[curr_ref_idx].len() > 1 {
                                return false;
                            }
                        }
                        Some(PacketValue::Val(l)) => return l < r,
                        _ => unreachable!(),
                    }

                    let curr_lhs_ref_value_idx = lhs_ref_value_idx.last_mut().unwrap();
                    *curr_lhs_ref_value_idx += 1;
                    let curr_rhs_ref_value_idx = rhs_ref_value_idx.last_mut().unwrap();
                    *curr_rhs_ref_value_idx += 1;
                }
                (Some(PacketValue::Val(l)), Some(PacketValue::Ref(next_lhs_ref_idx))) => {
                    let mut curr_ref_idx = *next_lhs_ref_idx;
                    while let Some(PacketValue::Ref(next_ref_idx)) = rhs.values[curr_ref_idx].get(0)
                    {
                        curr_ref_idx = *next_ref_idx;
                    }

                    match rhs.values[curr_ref_idx].get(0) {
                        None => return false,
                        Some(PacketValue::Val(r)) if l == r => {
                            if rhs.values[curr_ref_idx].len() > 1 {
                                return true;
                            }
                        }
                        Some(PacketValue::Val(r)) => return l < r,
                        _ => unreachable!(),
                    }

                    let curr_lhs_ref_value_idx = lhs_ref_value_idx.last_mut().unwrap();
                    *curr_lhs_ref_value_idx += 1;
                    let curr_rhs_ref_value_idx = rhs_ref_value_idx.last_mut().unwrap();
                    *curr_rhs_ref_value_idx += 1;
                }
                (None, Some(_)) => {
                    return true;
                }
                (Some(_), None) => return false,
                (None, None) => {
                    lhs_ref_idx.pop();
                    rhs_ref_idx.pop();
                    lhs_ref_value_idx.pop();
                    rhs_ref_value_idx.pop();
                }
            }
        }
    }
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<Vec<PacketValue>> = vec![];
        let chars = s.chars().collect::<Vec<_>>();
        let mut current_char = 0;
        let mut currently_editing_idxs = vec![];
        while current_char < chars.len() {
            match chars[current_char] {
                ',' => {
                    current_char += 1;
                }
                '[' => {
                    if let Some(last_editing_idx) = currently_editing_idxs.last() {
                        let next_editing_idx = stack.len();
                        let last_editing: &mut Vec<PacketValue> =
                            stack.get_mut(*last_editing_idx).unwrap();
                        last_editing.push(PacketValue::Ref(next_editing_idx));
                        currently_editing_idxs.push(next_editing_idx);
                    } else {
                        currently_editing_idxs.push(0);
                    }
                    stack.push(vec![]);
                    current_char += 1;
                }
                ']' => {
                    currently_editing_idxs.pop();
                    current_char += 1;
                }
                _ => {
                    let mut next_char = current_char + 1;
                    while chars[next_char].is_numeric() {
                        next_char += 1;
                    }

                    let x = String::from_iter(&chars[current_char..next_char])
                        .parse()
                        .unwrap();

                    let last_editing = stack
                        .get_mut(*currently_editing_idxs.last().unwrap())
                        .unwrap();
                    last_editing.push(PacketValue::Val(x));

                    current_char = next_char;
                }
            }
        }

        Ok(Self {
            values: stack,
            divider: false,
        })
    }
}

fn main() {
    let packet_data = common::io::collect_stdin_lines::<String>("\n\n");

    let mut part_1_data = vec![];
    let mut part_2_data = vec![];

    for v in packet_data {
        let p1 = v[0].parse::<Packet>().unwrap();
        let p2 = v[1].parse::<Packet>().unwrap();

        part_1_data.push((p1.clone(), p2.clone()));
        part_2_data.push(p1);
        part_2_data.push(p2);
    }

    let part_1 = part_1_data
        .iter()
        .enumerate()
        .filter(|(_, (p1, p2))| p1.compare(p2))
        .map(|(idx, _)| idx + 1)
        .sum::<usize>();
    println!("{}", part_1);

    part_2_data.push(Packet {
        values: vec![vec![PacketValue::Ref(1)], vec![PacketValue::Val(2)]],
        divider: true,
    });
    part_2_data.push(Packet {
        values: vec![vec![PacketValue::Ref(1)], vec![PacketValue::Val(6)]],
        divider: true,
    });

    part_2_data.sort_by(|lhs, rhs| {
        if lhs.compare(rhs) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    let part_2 = part_2_data
        .iter()
        .enumerate()
        .filter(|(_, p)| p.is_divider())
        .map(|(idx, _)| idx + 1)
        .product::<usize>();

    println!("{}", part_2)
}
