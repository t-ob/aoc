use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::AddX(_) => 2,
        }
    }
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
        let row = String::from_iter(row.iter().map(|b| if *b == 0 { '.' } else { '#' }));
        println!("{}", row);
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

        if pc == next_ready {
            if let Some(&Instruction::AddX(x)) = executing_instruction {
                reg_x += x;
            }
            let next_instruction = &program[i];
            next_ready = pc + next_instruction.cycles();
            executing_instruction = Some(next_instruction);

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
