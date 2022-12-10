use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();
        match (tokens.next(), tokens.next()) {
            (Some("addx"), Some(x)) => Ok(Self::AddX(x.parse().unwrap())),
            (Some("noop"), None) => Ok(Self::Noop),
            _ => unreachable!(),
        }
    }
}

fn render_crt(crt: &[u8; 240]) {
    for row in crt.chunks_exact(40) {
        let aaaaa = String::from_iter(row.iter().map(|b| if *b == 0 { '.' } else { '#' }));
        println!("{}", aaaaa);
    }
}

fn main() {
    let program = common::io::read_stdin_lines_to_vec::<Instruction>();

    let mut signal_strengths = vec![];
    let mut crt = [0u8; 240];

    let mut reg_x = 1;
    let mut pc = 0;
    let mut executing_instruction = None;
    let mut next_ready = 0;
    let mut i = 0;
    while i < program.len() {
        if pc >= 20 && (pc - 20) % 40 == 0 {
            signal_strengths.push((pc as i32) * reg_x);
        }

        let ready = pc == next_ready;
        if ready {
            if let Some(&Instruction::AddX(x)) = executing_instruction {
                reg_x += x;
            }
            next_ready = match program[i] {
                Instruction::Noop => pc + 1,
                Instruction::AddX(_) => pc + 2,
            };
            executing_instruction = Some(&program[i]);

            i += 1;
        }

        let pixel = (pc % 40) as i32;

        crt[pc] = ((reg_x - pixel).abs() <= 1) as u8;

        pc += 1;
    }

    let part_1 = signal_strengths.iter().sum::<i32>();

    println!("{}", part_1);

    render_crt(&crt);
}
