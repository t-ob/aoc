use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Operand {
    Number(u64),
    Old,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Operand),
    Mul(Operand),
}

impl Operation {
    fn apply(&self, x: u64) -> u64 {
        match self {
            Self::Add(Operand::Number(y)) => x + y,
            Self::Add(Operand::Old) => x + x,
            Self::Mul(Operand::Number(y)) => x * y,
            Self::Mul(Operand::Old) => x * x,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Test {
    DivisibleBy(u64, usize, usize),
}

impl Test {
    fn eval(&self, w: u64) -> usize {
        match self {
            Self::DivisibleBy(d, s, t) => {
                if w % *d == 0 {
                    *s
                } else {
                    *t
                }
            }
        }
    }
}

enum MonkeyBusinessMode {
    Part1,
    Part2(u64),
}

fn monkey_business(
    mode: MonkeyBusinessMode,
    rounds: usize,
    items: &Vec<VecDeque<u64>>,
    operations: &[Operation],
    tests: &[Test],
) -> u64 {
    let monkeys = items.len();

    let mut items = items.clone();

    let mut tmp_item_queues = vec![VecDeque::new(); monkeys];
    let mut inspections = vec![0; monkeys];

    let mut round = 0;
    while round < rounds {
        let mut monkey = 0;
        while monkey < monkeys {
            for item in items[monkey].drain(..) {
                let new_value = match mode {
                    MonkeyBusinessMode::Part1 => operations[monkey].apply(item) / 3,
                    MonkeyBusinessMode::Part2(modulus) => operations[monkey].apply(item) % modulus,
                };
                let destination = tests[monkey].eval(new_value);
                tmp_item_queues[destination].push_back(new_value);
                inspections[monkey] += 1;
            }

            for (dest_queue, src_queue) in items.iter_mut().zip(tmp_item_queues.iter_mut()) {
                for item in src_queue.drain(..) {
                    dest_queue.push_back(item);
                }
            }

            monkey += 1;
        }

        round += 1;
    }

    inspections.sort();
    inspections.pop().unwrap() * inspections.pop().unwrap()
}

fn main() {
    let groups = common::io::split_stdin("\n\n");

    let mut items = vec![];
    let mut operations = vec![];
    let mut tests = vec![];

    for group in groups {
        let lines = group.lines().collect::<Vec<_>>();

        items.push(VecDeque::<u64>::from_iter(
            lines[1][18..].split(", ").map(|s| s.parse().unwrap()),
        ));

        let operand = match &lines[2][25..] {
            "old" => Operand::Old,
            s => Operand::Number(s.parse().unwrap()),
        };

        let operation = match lines[2].chars().nth(23).unwrap() {
            '+' => Operation::Add(operand),
            '*' => Operation::Mul(operand),
            _ => unreachable!(),
        };

        operations.push(operation);

        let test_val = lines[3][21..].parse().unwrap();
        let test_outcome_true = lines[4][29..].parse().unwrap();
        let test_outcome_false = lines[5][30..].parse().unwrap();

        tests.push(Test::DivisibleBy(
            test_val,
            test_outcome_true,
            test_outcome_false,
        ))
    }

    let part_1 = monkey_business(MonkeyBusinessMode::Part1, 20, &items, &operations, &tests);
    println!("{:?}", part_1);

    let modulus = &tests
        .iter()
        .map(|t| match t {
            Test::DivisibleBy(d, _, _) => d,
        })
        .product();
    let part_2 = monkey_business(
        MonkeyBusinessMode::Part2(*modulus),
        10000,
        &items,
        &operations,
        &tests,
    );
    println!("{:?}", part_2);
}
