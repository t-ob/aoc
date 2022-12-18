use std::{cmp::max, str::FromStr};

#[derive(Clone, Copy)]
enum Push {
    Left,
    Right,
}

struct Chamber {
    jet_pattern: Vec<Push>,
    shapes: Vec<Vec<u8>>,
    data: Vec<u8>,
    next_push: usize,
    next_shape: usize,
    max_height: usize,
}

impl Chamber {
    fn new(jet_pattern: &[Push], shapes: &[Vec<u8>]) -> Self {
        let jet_pattern = Vec::from(jet_pattern);
        let shapes = Vec::from(shapes);
        let data = vec![0; 1 << 20];
        let next_push = 0;
        let next_shape = 0;
        let max_height = 0;

        Self { jet_pattern, shapes, data, next_push, next_shape, max_height }
    }

    fn drop_shape(&mut self) {
        let mut falling_shape = self.shapes[self.next_shape].clone();
        let mut height = self.max_height + 3;

        println!("{:?}, {:?}", falling_shape, height);

        let mut can_move_down = true;
        while can_move_down {
            self.apply_push(&mut falling_shape, height);

            let moved_down = self.apply_down(&falling_shape, height);
            if moved_down {
                height -= 1;
            } else {
                println!("Shape {:?} unable to move down at {height}", falling_shape);
                can_move_down = false;
            }
        }

        self.max_height = max(self.max_height, height + falling_shape.len());
        self.next_shape = (self.next_shape + 1) % self.shapes.len();
    }

    fn apply_push(&mut self, shape: &mut Vec<u8>, height: usize) {
        let push = self.jet_pattern[self.next_push];
        
        // apply_push(&mut chamber, height, &mut falling_shape, push);
        let new_shape = match push {
            Push::Left => shape.iter().map(|b| 0x7F & (*b << 1)).collect::<Vec<_>>(),
            Push::Right => shape.iter().map(|b| 0x7F & (*b >> 1)).collect::<Vec<_>>()
        };
    
        let mut can_push = true;
        for (h, (l, r)) in new_shape.iter().zip(shape.iter()).enumerate() {
            can_push &= l.count_ones() == r.count_ones(); // Didn't go off edge
            can_push &= *l & self.data[height + shape.len() - 1 - h] == 0 // Will occupy empty space
        }
    
        if can_push {
            shape.copy_from_slice(&new_shape)
        }
        
        self.next_push = (self.next_push + 1) % self.jet_pattern.len();
    }

    fn apply_down(&mut self, shape: &Vec<u8>, height: usize) -> bool {
        if height == 0 {
            for (i, r) in shape.iter().enumerate() {
                let h = height + shape.len() - 1 - i;
                self.data[h] |= *r;
            }
    
            return false
        }
    
        // shape.copy_from_slice(&new_shape);
    
        let mut can_move_down = true;
        for (i, r) in shape.iter().enumerate() {
            let h = height + shape.len() - 1 - i;
            can_move_down &= self.data[h - 1] & *r == 0;
        }
    
        if !can_move_down {
            for (i, r) in shape.iter().enumerate() {
                let h = height + shape.len() - 1 - i;
                self.data[h] |= *r;
            }
        }
    
        can_move_down
    }
}

fn main() {
    let jet_pattern = common::io::read_stdin::<String>()
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Push::Right,
            '<' => Push::Left,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    // let mut chamber = vec![0u8; 1 << 20];

    // let ch: [u8; 4] = [1, 2, 3, 4];

    // let mut wow = jet_pattern.iter().cycle();

    let shapes = vec![
        vec![0x1E], // [30]
        vec![0x8, 0x1C, 0x8], // [8, 28, 8] 
        vec![0x4, 0x4, 0x1C], // [4, 4, 28]
        vec![0x10, 0x10, 0x10, 0x10], // [16, 16, 16, 16]
        vec![0x18, 0x18], // [24, 24]
    ];

    let mut chamber = Chamber::new(&jet_pattern, &shapes);

    // let cols = vec![
    //     vec![0x10, 0x8, 0x4, 0x2],

    // ];

    let mut s = 0;
    let mut max_height = 0;
    while s < 2022 {
        chamber.drop_shape();

        s += 1;

        println!("{} - {}", chamber.max_height, chamber.data[chamber.max_height]);
    }

    for (i, r) in chamber.data.iter().enumerate() {
        if *r == 0x7F {
            println!("filled: {}", i);
        }
    }
}
