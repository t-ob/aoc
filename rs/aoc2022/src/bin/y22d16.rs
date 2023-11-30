use std::{
    cmp::max,
    collections::{HashMap, HashSet, VecDeque}
};

fn main() {
    let data = common::io::map_stdin_lines_to_vec(|line| {
        let mut groups = line.split("; ");
        let valve_group = groups.next().unwrap();
        let tunnels_group = groups.next().unwrap().splitn(5, ' ').nth(4).unwrap();
        let valve_chars = valve_group.chars().collect::<Vec<_>>();

        let valve = 26 * (valve_chars[6] as usize - b'A' as usize)
            + (valve_chars[7] as usize - b'A' as usize);
        let flow_rate = String::from_iter(&valve_chars[23..])
            .parse::<u32>()
            .unwrap();

        let flows_to = tunnels_group
            .split(", ")
            .map(|s| {
                let mut valve_chars = s.chars();

                let valve = 26 * (valve_chars.next().unwrap() as u8 - b'A') as usize
                    + (valve_chars.next().unwrap() as u8 - b'A') as usize;

                valve
            })
            .collect::<Vec<_>>();

        (valve, flow_rate, flows_to)
    });

    let mut flow_rates = vec![0; 1 << 10];
    let mut tunnels = vec![vec![]; 1 << 10];

    let mut known_valves = vec![];

    for (valve, flow_rate, mut flows_to) in data {
        known_valves.push(valve);
        flow_rates[valve] = flow_rate;
        tunnels[valve].append(&mut flows_to);
    }

    let mut onon = HashMap::new();

    for valve in &known_valves {
        let mut seen = HashSet::new();
        seen.insert(*valve);
        let mut queue = VecDeque::new();
        let mut requeue = VecDeque::new();
        queue.push_back(*valve);

        let mut noon = vec![];

        let mut d = 0u32;
        while !queue.is_empty() {
            d += 1;

            for n in queue.drain(..) {
                for nn in &tunnels[n] {
                    if seen.contains(&nn) {
                        continue;
                    }
                    seen.insert(*nn);
                    requeue.push_back(*nn);
                    if flow_rates[*nn] == 0 {
                        continue;
                    }
                    noon.push((*nn, d));
                }
            }

            queue.append(&mut requeue);
        }

        if *valve != 0 && flow_rates[*valve] == 0 {
            continue;
        }
        onon.insert(*valve, noon);
    }

    let oioion = onon.clone();
    let uuuuu1: HashSet<usize> = oioion.keys().map(|v| *v).collect(); 

    let mut fuck1 = Fuck {
        graph: oioion,
        flows: flow_rates.clone(),
        target_minute: 30,
        unopened: uuuuu1,
        pressure: 0,
        best_pressure: 0,
        actors: 1,
        minute: vec![0; 1],
        path: vec![vec![0]; 1],
        best_path: vec![vec![0]; 1],
        seen: vec![HashSet::new(); 1],
    };

    fuck1.fuck();

    println!("{:#?}", fuck1.best_pressure);

    let opnon = onon.clone();

    let uuuuu: HashSet<usize> = opnon.keys().map(|v| *v).collect(); 

    let mut fuuuuck = Fuck {
        graph: opnon,
        flows: flow_rates.clone(),
        target_minute: 26,
        unopened: uuuuu,
        pressure: 0,
        best_pressure: 0,
        actors: 2,
        minute: vec![0; 2],
        path: vec![vec![0]; 2],
        best_path: vec![vec![0]; 2],
        seen: vec![HashSet::new(); 2],
    };

    fuuuuck.fuck();

    println!("{:#?}", fuuuuck.best_pressure);
}


struct Graph {
    adjacency_list: HashMap<usize, Vec<(usize, u32)>>,
    flow_rates: HashMap<usize, u32>,
}


#[derive(Debug)]
struct Fuck {
    graph: HashMap<usize, Vec<(usize, u32)>>,
    flows: Vec<u32>,
    target_minute: u32,
    unopened: HashSet<usize>,
    pressure: u32,
    best_pressure: u32,
    actors: usize,
    minute: Vec<u32>,
    path: Vec<Vec<usize>>,
    best_path: Vec<Vec<usize>>,
    seen: Vec<HashSet::<(usize, u32, (u128, u128, u128, u128, u128, u128))>>
}

impl Fuck {
    fn sig(&self, a: usize) -> (usize, u32, (u128, u128, u128, u128, u128, u128)) {
        let mut s = [0; 6];
        for u in &self.unopened {
            let i = u / 128;
            let o = u % 128;
            s[i] |= 1 << o;
        }

        (*self.path[a].last().unwrap(), self.minute[a], (s[0], s[1], s[2], s[3], s[4], s[5]))
    }

    fn fuck(&mut self) {
        let mmm = max(self.best_pressure, self.pressure);
        if mmm > self.best_pressure {
            self.best_pressure = mmm;
            for i in 0..self.actors {
                self.best_path[i] = self.path[i].clone();
            }
        }

        for i in 0..self.actors {
            let s = self.sig(i);
            if self.seen[i].contains(&s) {
                continue;
            }
            
            let ns = self.graph.get(&self.path[i].last().unwrap()).unwrap().iter().filter(|(n, d)| {
                self.unopened.contains(n) && self.minute[i] + *d + 1 <= self.target_minute
            }).map(|x| *x).collect::<Vec<_>>();

    
            for (n, d) in ns {
                let pressure_contributed = (self.target_minute - (&self.minute[i] + d + 1)) * self.flows[n];
    
                self.minute[i] += d + 1;
                self.pressure += pressure_contributed;
                self.unopened.remove(&n);
                self.path[i].push(n);
    
                self.fuck();

                let gon = self.sig(i);
                self.seen[i].insert(gon);
    
                self.path[i].pop();
                self.unopened.insert(n);
                self.pressure -= pressure_contributed;
                self.minute[i] -= d + 1;
            }
        }
    }
}
