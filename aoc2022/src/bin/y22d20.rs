#[derive(Debug)]
struct Shit {
    val: i32,
    prev: usize,
    next: usize,
}

fn main() {
    let nums = common::io::read_stdin_lines_to_vec::<i32>();
    let foo = nums.iter().enumerate().collect::<Vec<_>>();

    let fuck = nums.len() as i32;
    let fuck_nums = nums
        .iter()
        .map(|n| ((*n % fuck) + fuck) % fuck)
        .collect::<Vec<_>>();

    println!("{:?} {:?}", nums.iter().min(), nums.iter().max());

    println!("{:?}", fuck_nums);

    let shit = *fuck_nums.iter().max().unwrap() as usize;

    let mut prick = (0..nums.len()).collect::<Vec<_>>();

    let mut shits = nums
        .iter()
        .enumerate()
        .map(|(i, n)| {
            let m = nums.len();
            let val = *n;
            let prev = ((((i as i32 - 1) % m as i32) + m as i32) % m as i32) as usize;
            let next = (i + 1) % m;
            Shit { val, next, prev }
        })
        .collect::<Vec<_>>();

    let m = shits.len() as i32;
    for i in 0..1 {
        let s = &shits[i];
        let r = ((s.val % m) + m) % m;

        if r == 0 {
            continue;
        }
        
        let prev = s.prev;
        let mut fuck = s.next;
        let mut rr = 0;
        while rr < r - 1 {
            let ss = &shits[fuck];
            fuck = ss.next;
            rr += 1;
        }

        shits[prev].next = shits[i].next;
        shits[i].next = shits[fuck].next;
        shits[i].prev = fuck;
        shits[fuck].next = i;
        shits[fuck].prev = prev;
    }

    println!("{:?}", shits);
    //   *
    // [ 0 ]     [ 1 ]     [ 2 ]     [ 3 ]     [ 4 ]     [ 5 ]     [ 6 ]
    // [ 1 | 1 ] [ 2 | 2 ] [ 4 | 3 ] [ 3 | 4 ] [ 5 | 5 ] [ 0 | 6 ] [ 4 | 0 ]

    // head = 0

    // step 1
    // node [0] next becomes next of 1 forward, prev next becomes [ 0 ]:

    //       |-----( i )-----|
    // [ 0 ]     [ 1 ]     [ 2 ]     [ 3 ]     [ 4 ]     [ 5 ]     [ 6 ]
    // [ 1 | 2 ] [ 2 | 0 ] [ 4 | 3 ] [ 3 | 4 ] [ 5 | 5 ] [ 0 | 6 ] [ 4 | 0 ]
    //       |-( ii )--|
    // repr: 2 1 4 3 5 0 4

    // step 2
    // node [1] next becomes next of 2 forward, prev next becomes [ 1 ]:

    //       |-------------------|
    //       |---------|
    // [ 0 ]     [ 1 ]     [ 2 ]     [ 3 ]     [ 4 ]     [ 5 ]     [ 6 ]
    // [ 1 | 2 ] [ 2 | 3 ] [ 4 | 1 ] [ 3 | 4 ] [ 5 | 5 ] [ 0 | 6 ] [ 4 | 0 ]
    //                 |-( ii )--|
    // repr: 1 4 2 3 5 0 4
}
